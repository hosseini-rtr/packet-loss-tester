import { useCallback, useEffect, useRef, useState } from "react";
import type { TestResult, TestSession, TestSettings } from "../types";

const WS_URL = "ws://localhost:8080";

interface PacketStats {
  messagesReceived: number;
  messagesMissed: number;
  packetLossPercentage: number;
  lastSequence: number;
}

export const usePacketTest = () => {
  const [session, setSession] = useState<TestSession | null>(null);
  const [error, setError] = useState<string | null>(null);
  const [ws, setWs] = useState<WebSocket | null>(null);
  const sequenceCounter = useRef(0);
  const intervalRef = useRef<number | null>(null);

  const startTest = async (settings: TestSettings) => {
    try {
      setError(null);
      sequenceCounter.current = 0;

      // Create new session
      const sessionId = Date.now().toString();
      setSession({
        id: sessionId,
        settings,
        results: [],
        status: "running",
      });

      // Connect to WebSocket
      const wsConnection = new WebSocket(WS_URL);
      setWs(wsConnection);

      // Setup message sending interval
      wsConnection.onopen = () => {
        intervalRef.current = window.setInterval(() => {
          if (wsConnection.readyState === WebSocket.OPEN) {
            const message = {
              seq: sequenceCounter.current,
              ts: Date.now(),
            };
            wsConnection.send(JSON.stringify(message));
            sequenceCounter.current++;
          }
        }, 1000 / settings.frequency); // Convert frequency to milliseconds interval
      };
    } catch (err) {
      setError(err instanceof Error ? err.message : "Failed to start test");
      setSession(null);
    }
  };

  const stopTest = useCallback(() => {
    if (intervalRef.current) {
      clearInterval(intervalRef.current);
      intervalRef.current = null;
    }
    if (ws) {
      ws.close();
      setWs(null);
    }
    if (session) {
      setSession({ ...session, status: "completed" });
    }
  }, [ws, session]);

  useEffect(() => {
    if (!ws) return;

    ws.onmessage = (event) => {
      try {
        const message = JSON.parse(event.data);
        const now = Date.now();
        const rtt = now - message.ts; // Calculate Round Trip Time

        const result: TestResult = {
          timestamp: now,
          rtt,
          sequence: message.seq,
        };

        setSession((prev) => {
          if (!prev) return null;
          return {
            ...prev,
            results: [...prev.results, result],
          };
        });
      } catch (err) {
        console.error("Failed to parse WebSocket message:", err);
      }
    };

    ws.onerror = (event) => {
      console.error("WebSocket error:", event);
      setError("WebSocket connection error");
      stopTest();
    };

    ws.onclose = () => {
      setWs(null);
      if (intervalRef.current) {
        clearInterval(intervalRef.current);
        intervalRef.current = null;
      }
      setSession((prev) => (prev ? { ...prev, status: "completed" } : null));
    };

    return () => {
      if (intervalRef.current) {
        clearInterval(intervalRef.current);
        intervalRef.current = null;
      }
      ws.close();
    };
  }, [ws, stopTest]);

  // Calculate packet loss stats
  const stats = useCallback((): PacketStats | null => {
    if (!session?.results.length) return null;

    const received = new Set(session.results.map((r) => r.sequence));
    const maxSeq = Math.max(...Array.from(received));
    const missed = maxSeq + 1 - received.size;
    const total = maxSeq + 1;

    return {
      messagesReceived: received.size,
      messagesMissed: missed,
      packetLossPercentage: (missed / total) * 100,
      lastSequence: maxSeq,
    };
  }, [session?.results]);

  return {
    session,
    error,
    isRunning: session?.status === "running",
    startTest,
    stopTest,
    stats: stats(),
  };
};

export default usePacketTest;

import React from "react";
import {
  CartesianGrid,
  Line,
  LineChart,
  ResponsiveContainer,
  Tooltip,
  XAxis,
  YAxis,
} from "recharts";
import { TestResult } from "../types";

interface Props {
  results: TestResult[];
  isRunning: boolean;
  stats: {
    messagesReceived: number;
    messagesMissed: number;
    packetLossPercentage: number;
    lastSequence: number;
  } | null;
}

export const TestResults: React.FC<Props> = ({ results, isRunning, stats }) => {
  const chartData = results.map((result) => ({
    time: new Date(result.timestamp).toLocaleTimeString(),
    rtt: result.rtt,
    sequence: result.sequence,
  }));

  return (
    <div className="bg-white dark:bg-gray-800 p-6 rounded-lg shadow-lg">
      <h2 className="text-2xl font-bold mb-6 text-gray-900 dark:text-white">
        Test Results {isRunning && "(Live)"}
      </h2>

      {/* Stats Cards */}
      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4 mb-6">
        <div className="bg-blue-50 dark:bg-blue-900 p-4 rounded-lg">
          <h3 className="text-sm font-medium text-blue-800 dark:text-blue-200">
            Messages Received
          </h3>
          <p className="mt-2 text-3xl font-semibold text-blue-900 dark:text-blue-100">
            {stats?.messagesReceived || 0}
          </p>
        </div>

        <div className="bg-red-50 dark:bg-red-900 p-4 rounded-lg">
          <h3 className="text-sm font-medium text-red-800 dark:text-red-200">
            Messages Missed
          </h3>
          <p className="mt-2 text-3xl font-semibold text-red-900 dark:text-red-100">
            {stats?.messagesMissed || 0}
          </p>
        </div>

        <div className="bg-yellow-50 dark:bg-yellow-900 p-4 rounded-lg">
          <h3 className="text-sm font-medium text-yellow-800 dark:text-yellow-200">
            Packet Loss
          </h3>
          <p className="mt-2 text-3xl font-semibold text-yellow-900 dark:text-yellow-100">
            {stats?.packetLossPercentage.toFixed(2)}%
          </p>
        </div>

        <div className="bg-green-50 dark:bg-green-900 p-4 rounded-lg">
          <h3 className="text-sm font-medium text-green-800 dark:text-green-200">
            Average RTT
          </h3>
          <p className="mt-2 text-3xl font-semibold text-green-900 dark:text-green-100">
            {results.length
              ? (
                  results.reduce((acc, r) => acc + r.rtt, 0) / results.length
                ).toFixed(2)
              : "0"}{" "}
            ms
          </p>
        </div>
      </div>

      {/* RTT Chart */}
      <div className="h-80 mt-6">
        <ResponsiveContainer width="100%" height="100%">
          <LineChart data={chartData}>
            <CartesianGrid strokeDasharray="3 3" />
            <XAxis
              dataKey="time"
              label={{
                value: "Time",
                position: "insideBottom",
                offset: -10,
              }}
            />
            <YAxis
              label={{
                value: "RTT (ms)",
                angle: -90,
                position: "insideLeft",
              }}
            />
            <Tooltip />
            <Line
              type="monotone"
              dataKey="rtt"
              stroke="#3b82f6"
              dot={false}
              name="RTT"
            />
          </LineChart>
        </ResponsiveContainer>
      </div>

      {/* Sequence Chart */}
      <div className="h-80 mt-6">
        <ResponsiveContainer width="100%" height="100%">
          <LineChart data={chartData}>
            <CartesianGrid strokeDasharray="3 3" />
            <XAxis
              dataKey="time"
              label={{
                value: "Time",
                position: "insideBottom",
                offset: -10,
              }}
            />
            <YAxis
              label={{
                value: "Sequence Number",
                angle: -90,
                position: "insideLeft",
              }}
            />
            <Tooltip />
            <Line
              type="monotone"
              dataKey="sequence"
              stroke="#10b981"
              dot={false}
              name="Sequence"
            />
          </LineChart>
        </ResponsiveContainer>
      </div>
    </div>
  );
};

export default TestResults;

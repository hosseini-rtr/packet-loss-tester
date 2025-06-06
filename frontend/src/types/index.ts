export interface TestSettings {
  packet_size: number;
  frequency: number;
  duration: number;
  acceptable_delay: number;
  wait_before_recording: boolean;
  preset?: "custom" | "gaming" | "video_call";
}

export interface TestResult {
  timestamp: number;
  sequence: number;
  rtt: number;
}

export interface TestSession {
  id: string;
  settings: TestSettings;
  results: TestResult[];
  status: "running" | "completed";
}

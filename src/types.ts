export interface HostInfo {
  alias: string;
  host: string;
  paused?: boolean;
  continued?: boolean;
}

export interface Host extends HostInfo {
  pingResults: PingResult[];
  lastPing: PingResult;
}

export interface PingResult {
  host: string;
  duration: number;
  timestamp: number;
  status: string;
}

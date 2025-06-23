export interface HostInfo {
  id: string;
  host: string;
  alias: string;
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

export interface Dashboard {
  name: string;
  hosts: string[];
}

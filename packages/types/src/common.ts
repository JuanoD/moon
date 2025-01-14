import type { PlatformType } from './tasks-config';

export type Nullable<T> = { [K in keyof T]: T[K] | null };

export interface Duration {
	secs: number;
	nanos: number;
}

export interface Runtime {
	platform: Capitalize<PlatformType>;
	version?: string;
}

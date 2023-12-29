export type PulseType = 'low' | 'high';

export class FlipFlopModule {
	name: string;
	type: '%';
	targets: string[];
	memory: 'on' | 'off';

	constructor(name: string, type: '%', targets: string[]) {
		this.name = name;
		this.type = type;
		this.targets = targets;
		this.memory = 'off';
	}
}

export class ConjunctionModule {
	name: string;
	type: '&';
	targets: string[];
	memory: Record<string, PulseType>;

	constructor(name: string, type: '&', targets: string[]) {
		this.name = name;
		this.type = type;
		this.targets = targets;
		this.memory = {};
	}
}

export interface Pulse {
	origin: string;
	target: string;
	type: PulseType;
}

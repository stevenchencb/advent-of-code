import { getFileLines } from '../utils';
import { ConjunctionModule, FlipFlopModule, Pulse } from './models';

const input_lines = await getFileLines(20);
const modules: Record<string, FlipFlopModule | ConjunctionModule> = {};
const broadcastTargets: string[] = [];
let lowPulses = 0;
let highPulses = 0;

export async function solve() {
	processInput();

	for (let i = 0; i < 1000; i++) {
		lowPulses++;

		const queue: Pulse[] = broadcastTargets.map((t) => ({ origin: 'broadcaster', target: t, type: 'low' }));

		while (queue.length) {
			const pulse = queue.shift()!;
			const targetModule = modules[pulse.target];

			pulse.type === 'low' ? lowPulses++ : highPulses++;

			if (targetModule instanceof FlipFlopModule) {
				const nextPulses = processPulseForFlipFlopModule(targetModule, pulse);
				queue.push(...nextPulses);
			} else if (targetModule instanceof ConjunctionModule) {
				const nextPulses = processPulseForConjunctionModule(targetModule, pulse);
				queue.push(...nextPulses);
			}
		}
	}

	return lowPulses * highPulses;
}

function processInput() {
	// setup modules
	for (const line of input_lines) {
		const [module, targets] = line.split(' -> ');
		const moduleTargets = targets.split(', ');

		if (module === 'broadcaster') {
			broadcastTargets.push(...moduleTargets);
		} else {
			const moduleName = module.substring(1);
			if (module.startsWith('%')) {
				modules[moduleName] = new FlipFlopModule(moduleName, '%', moduleTargets);
			} else {
				modules[moduleName] = new ConjunctionModule(moduleName, '&', moduleTargets);
			}
		}
	}

	// setup memories of conjunction modules
	for (const line of input_lines) {
		const namesAndTargets = line.split(' -> ');
		const moduleName = namesAndTargets[0] === 'broadcaster' ? 'broadcaster' : namesAndTargets[0].substring(1);
		const targetModules = namesAndTargets[1].split(', ');

		for (const target of targetModules) {
			const module = modules[target];
			if (module && module.type === '&') {
				module.memory[moduleName] = 'low';
			}
		}
	}
}

function processPulseForFlipFlopModule(module: FlipFlopModule, pulse: Pulse): Pulse[] {
	if (pulse.type === 'high') {
		return [];
	}

	if (module.memory === 'off') {
		module.memory = 'on';
		return module.targets.map((t) => ({ origin: module.name, target: t, type: 'high' }));
	} else {
		module.memory = 'off';
		return module.targets.map((t) => ({ origin: module.name, target: t, type: 'low' }));
	}
}

function processPulseForConjunctionModule(module: ConjunctionModule, pulse: Pulse): Pulse[] {
	module.memory[pulse.origin] = pulse.type;

	if (Object.values(module.memory).every((p) => p === 'high')) {
		return module.targets.map((t) => ({ origin: module.name, target: t, type: 'low' }));
	} else {
		return module.targets.map((t) => ({ origin: module.name, target: t, type: 'high' }));
	}
}

import { getFileLines } from '../utils';
import { ConjunctionModule, FlipFlopModule, Pulse } from './models';

const input_lines = await getFileLines(20);
const modules: Record<string, FlipFlopModule | ConjunctionModule> = {};
const broadcastTargets: string[] = [];
let buttonPresses = 0;

export async function solve() {
	processInput();

	// check modules which send pulses to the module that sends pulses to rx
	// e.g. all modules x for which  x -> y -> rx
	// idea: because y can only be a single module and y is a conjunction module,
	// calculate the lcm of the cycle lengths of all x when x is sending a high pulse
	const modulesToCheck = determineModulesToCheckForCycleLength();
	const cycleLengths: Record<string, number> = {};

	while (true) {
		buttonPresses++;
		const queue: Pulse[] = broadcastTargets.map((t) => ({ origin: 'broadcaster', target: t, type: 'low' }));

		while (queue.length) {
			const pulse = queue.shift()!;
			const targetModule = modules[pulse.target];

			// cycle length recording
			if (modulesToCheck.includes(pulse.origin) && pulse.type === 'high') {
				if (cycleLengths[pulse.origin] === undefined) {
					cycleLengths[pulse.origin] = buttonPresses;
				}

				// return lcm when we know the cycle lengths of all modules sending to the module sending to rx
				if (Object.entries(cycleLengths).length === modulesToCheck.length) {
					return lcm(...Object.values(cycleLengths));
				}
			}

			if (targetModule instanceof FlipFlopModule) {
				const nextPulses = processPulseForFlipFlopModule(targetModule, pulse);
				queue.push(...nextPulses);
			} else if (targetModule instanceof ConjunctionModule) {
				const nextPulses = processPulseForConjunctionModule(targetModule, pulse);
				queue.push(...nextPulses);
			}
		}
	}
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

function determineModulesToCheckForCycleLength(): string[] {
	const moduleToRx = Object.values(modules).find((m) => m.targets.includes('rx'))!;

	return Object.values(modules)
		.filter((m) => m.targets.includes(moduleToRx.name))
		.map((m) => m.name);
}

function lcm(...nums: number[]): number {
	const gcd = (x: number, y: number): number => (!y ? x : gcd(y, x % y));
	const _lcm = (x: number, y: number) => (x * y) / gcd(x, y);
	return [...nums].reduce((a, b) => _lcm(a, b));
}

import { getFileLines } from '../../utils';

const stacks: Record<number, string[]> = {
	1: ['B', 'Z', 'T'],
	2: ['V', 'H', 'T', 'D', 'N'],
	3: ['B', 'F', 'M', 'D'],
	4: ['T', 'J', 'G', 'W', 'V', 'Q', 'L'],
	5: ['W', 'D', 'G', 'P', 'V', 'F', 'Q', 'M'],
	6: ['V', 'Z', 'Q', 'G', 'H', 'F', 'S'],
	7: ['Z', 'S', 'N', 'R', 'L', 'T', 'C', 'W'],
	8: ['Z', 'H', 'W', 'D', 'J', 'N', 'R', 'M'],
	9: ['M', 'Q', 'L', 'F', 'D', 'S'],
};

const moveList = await getFileLines('./input.txt');

for (const moves of moveList) {
	const numOfMoves = moves.replace(/move (\d+).*/, '$1');
	const fromStack = moves.replace(/move \d+ from (\d+).*/, '$1');
	const toStack = moves.replace(/move \d+ from \d+ to (\d+).*/, '$1');
	for (let i = 0; i < Number(numOfMoves); i++) {
		stacks[Number(toStack)].push(stacks[Number(fromStack)].pop() ?? '');
	}
}

console.log(
	Object.values(stacks)
		.map((s) => s[s.length - 1])
		.join('')
);

import { getFileLines } from '../../utils';

const lines = await getFileLines('./input.txt');

const rounds = lines.map((x) => x.replace(/\s/g, ''));

const scoreMap: Record<string, number> = {
	AX: 4,
	AY: 8,
	AZ: 3,
	BX: 1,
	BY: 5,
	BZ: 9,
	CX: 7,
	CY: 2,
	CZ: 6,
};

const score = rounds.reduce((acc, curr) => {
	acc += scoreMap[curr];
	return acc;
}, 0);

console.log(score);

import { getFileLines } from '../../utils';

const lines = await getFileLines('./input.txt');

const result = lines.reduce((acc, curr) => {
	const assignments = curr.split(',');
	const assignmentA = assignments[0].split('-');
	const assignmentB = assignments[1].split('-');
	const hasOverlap = Math.max(Number(assignmentA[0]), Number(assignmentB[0])) <= Math.min(Number(assignmentA[1]), Number(assignmentB[1]));

	return hasOverlap ? acc + 1 : acc;
}, 0);

console.log(result);

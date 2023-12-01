import { getFileLines } from '../../utils';

const inputLines = await getFileLines('./input.txt');
const numbersRegex = /one|two|three|four|five|six|seven|eight|nine|[0-9]/gi;
const numberMap: Record<string, number> = {
	one: 1,
	two: 2,
	three: 3,
	four: 4,
	five: 5,
	six: 6,
	seven: 7,
	eight: 8,
	nine: 9,
};

const calibrationSum = inputLines.reduce((acc, curr) => {
	const numbersOfLine: string[] = [];
	let matches = numbersRegex.exec(curr);
	while (matches) {
		// matching "consumes" characters it matched
		// manually set lastIndex (where to start the next match) back in case of overlapping matches
		numbersRegex.lastIndex -= matches[0].length - 1;
		numbersOfLine.push(matches[0]);
		matches = numbersRegex.exec(curr);
	}
	const firstNumber = mapToNumber(numbersOfLine[0]);
	const lastNumber = mapToNumber(numbersOfLine[numbersOfLine.length - 1]);
	const calibration = Number.parseInt(`${firstNumber}${lastNumber}`);
	return acc + calibration;
}, 0);

console.log(calibrationSum);

function mapToNumber(s: string | undefined) {
	if (!s) return undefined;
	const parsed = Number.parseInt(s);
	return Number.isInteger(parsed) ? parsed : numberMap[s];
}

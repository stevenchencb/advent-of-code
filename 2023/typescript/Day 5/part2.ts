import { extractOne, getFileInput } from '../utils';

const input = await getFileInput('./input.txt');
const seedsAndMapsStrings = input.split('\n\n');

const seedToSoilMap = getMap(seedsAndMapsStrings, 'seed-to-soil');
const soilToFertMap = getMap(seedsAndMapsStrings, 'soil-to-fertilizer');
const fertToWaterMap = getMap(seedsAndMapsStrings, 'fertilizer-to-water');
const waterToLightMap = getMap(seedsAndMapsStrings, 'water-to-light');
const lightToTempMap = getMap(seedsAndMapsStrings, 'light-to-temperature');
const tempToHumidMap = getMap(seedsAndMapsStrings, 'temperature-to-humidity');
const humidToLocationMap = getMap(seedsAndMapsStrings, 'humidity-to-location');

const seeds = [...seedsAndMapsStrings[0].matchAll(/\d+/g)].map((m) => Number.parseInt(m[0]));

const seedRanges = seeds.reduce<number[][]>((acc, curr, i) => {
	if (i % 2 === 0) {
		acc[Math.floor(i / 2)] = [curr];
	} else {
		acc[Math.floor(i / 2)][1] = curr;
	}
	return acc;
}, []);

console.log(findSmallestLocationNumber());

function findSmallestLocationNumber(): number {
	for (let i = 0; i < Number.MAX_SAFE_INTEGER; i++) {
		const seed = reverseMap(
			reverseMap(
				reverseMap(
					reverseMap(reverseMap(reverseMap(reverseMap(i, humidToLocationMap), tempToHumidMap), lightToTempMap), waterToLightMap),
					fertToWaterMap
				),
				soilToFertMap
			),
			seedToSoilMap
		);

		for (const range of seedRanges) {
			if (seed >= range[0] && seed < range[0] + range[1]) {
				return i;
			}
		}
	}
	return -1;
}

function getMap(seedsAndMapsStrings: string[], mapName: string) {
	const mapLines =
		seedsAndMapsStrings
			.find((s) => s.includes(mapName))
			?.split('\n')
			?.slice(1) ?? [];
	const resultMap: number[][] = [];

	for (const line of mapLines) {
		const regex = /(?<destStart>\d+) (?<srcStart>\d+) (?<range>\d+)/;
		const destStart = Number.parseInt(extractOne(line, regex, 'destStart'));
		const srcStart = Number.parseInt(extractOne(line, regex, 'srcStart'));
		const range = Number.parseInt(extractOne(line, regex, 'range'));

		resultMap.push([srcStart, destStart, range]);
	}

	return resultMap;
}

function reverseMap(n: number, map: number[][]): number {
	for (const srcDestRange of map) {
		if (srcDestRange[1] <= n && srcDestRange[1] + srcDestRange[2] - 1 >= n) {
			return n - srcDestRange[1] + srcDestRange[0];
		}
	}

	return n;
}

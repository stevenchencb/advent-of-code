export async function getFileInput(day: number): Promise<string> {
	return Bun.file(`./Day ${day}/input.txt`).text();
}

export async function getFileLines(day: number): Promise<string[]> {
	return (await Bun.file(`./Day ${day}/input.txt`).text()).split('\n');
}

export function extractOne(s: string, regexp: string | RegExp, groupToExtract: string | number, regexFlags = 'gi') {
	const regex = new RegExp(regexp, regexFlags);
	const matches = regex.exec(s);
	return (typeof groupToExtract === 'string' ? (matches?.groups ?? {})[groupToExtract] : matches?.at(groupToExtract)) ?? '';
}

export function extractMultiple<T extends readonly string[]>(
	s: string,
	regexp: string | RegExp,
	groupsToExtract: T,
	regexFlags = 'gi'
): Record<T[number], string>[] {
	const regex = new RegExp(regexp, regexFlags);
	const matches = [...s.matchAll(regex)];
	return matches.map((m) => {
		return groupsToExtract.reduce<Record<T[number], string>>((acc: Record<T[number], string>, curr: T[number]) => {
			acc[curr] = m.groups?.[curr] ?? '';
			return acc;
		}, {} as Record<T[number], string>);
	});
}

export function intersection<T>(a: T[], b: T[]) {
	return a.filter((x) => !!b.find((y) => x === y));
}

export function intersects(a: [number, number][], b: [number, number][]) {
	return a.filter((x) => b.find((y) => isEqualCoordinate(x, y))).length > 0;
}

export function isEqualCoordinate(a: [number, number], b: [number, number]) {
	return a[0] === b[0] && a[1] === b[1];
}

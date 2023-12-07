export type Card = '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' | 'T' | 'J' | 'Q' | 'K' | 'A';

export const CardOrder: Record<Card, number> = {
	'2': 1,
	'3': 2,
	'4': 3,
	'5': 4,
	'6': 5,
	'7': 6,
	'8': 7,
	'9': 8,
	T: 9,
	J: 10,
	Q: 11,
	K: 12,
	A: 13,
};

export const HandTypeOrder = {
	HIGH_CARD: 1,
	ONE_PAIR: 2,
	TWO_PAIR: 3,
	TRIPLET: 4,
	FULL_HOUSE: 5,
	QUADRUPLET: 6,
	QUINTUPLET: 7,
};
export type HandType = keyof typeof HandTypeOrder;

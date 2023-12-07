export type Card = '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' | 'T' | 'J' | 'Q' | 'K' | 'A';

export const CardOrder: Record<Card, number> = {
	J: 1,
	'2': 2,
	'3': 3,
	'4': 4,
	'5': 5,
	'6': 6,
	'7': 7,
	'8': 8,
	'9': 9,
	T: 10,
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

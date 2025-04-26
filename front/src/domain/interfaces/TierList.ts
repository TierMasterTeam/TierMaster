export interface TierList {
  id?: string;
  name: string;
  author: string;
  cards: Card[];
  grades: Grade[];

}

export interface Grade {
  name: string;
  color: string;
  cards: Card[];
}

export interface Card {
  name: string;
  image: string;
}

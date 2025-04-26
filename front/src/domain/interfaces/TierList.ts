export interface TierList {
  id?: string;
  name: string;
  author: string;
  cards: cards[];
  grades: grades[];

}

export interface grades {
  name: string;
  color: string;
  cards: cards[];
}

export interface cards {
  name: string;
  image: string;
}

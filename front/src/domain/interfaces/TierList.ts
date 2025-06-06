export interface TierList {
  id?: string;
  name: string;
  author: string;
  tags: string[];
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

export interface CardUpload {
  name: string;
  image: File;
}


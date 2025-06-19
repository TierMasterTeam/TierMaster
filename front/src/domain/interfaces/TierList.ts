export interface TierList {
  id?: string;
  name: string;
  author: string;
  imgCover?: string;
  is_public: boolean;
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


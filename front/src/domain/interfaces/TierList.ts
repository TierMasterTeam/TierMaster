export interface TierList {
  id?: string;
  name: string;
  author: string;
  coverImage?: string;
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

export interface GradesOnly {
  name: string;
  color: string;
}

export interface Card {
  name: string;
  image: string;
}

export interface CardUpload {
  name: string;
  image: File;
}

export interface Template {
  id?: string;
  name: string;
  author: string;
  coverImage?: string;
  is_public: boolean;
  tags: string[];
  cards: Card[];
  grades: GradesOnly[];
}

export interface SearchResult {
  page: number;
  per_page: number;
  data: Template[];
}


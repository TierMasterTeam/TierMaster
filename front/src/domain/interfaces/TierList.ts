import type { Item } from "./Item";

export interface TierList {
  name: string;
  tiers: Tier[];
}

export interface Tier {
  name: string;
  color: string;
  items: Item[];
}

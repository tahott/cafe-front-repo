import { StateUpdater } from "preact/hooks";

export interface Category {
  key: string;
  name: string;
}

export interface Menu {
  type: string;
  name: string;
  price: number;
}

export interface CartContext {
  cart: Menu[];
  setCart: StateUpdater<Menu[]>;
}
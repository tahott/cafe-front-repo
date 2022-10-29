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

export interface Cart extends Menu {
  amount: number;
}

export interface CartContext {
  cart: Map<string, Cart>;
  setCart: StateUpdater<Map<string, Cart>>;
}
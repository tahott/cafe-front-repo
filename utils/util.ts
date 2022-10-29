import { Cart } from "./types.tsx"

export const mapLoop = (data: Map<unknown, Cart>): number => {
  let total = 0;
  for (const [_, v] of data) {
    total += (v.price * v.amount);
  }

  return total;
}
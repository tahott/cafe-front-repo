import { useCart } from "../utils/hooks.tsx";
import { Cart } from "../utils/types.tsx";
import { mapLoop } from "../utils/util.ts";

enum Operator {
  ADD,
  SUB,
}

export function Cart() {
  const { cart, setCart } = useCart();

  const handleChangeCartAmount = (c: Cart, operator: Operator) => {
    const { type, name, price } = c;
    setCart((cart) => {
      if (!cart.has(name)) {
        cart.set(name, { type, name, price, amount: 0 })
      }

      const prev = cart.get(name);

      cart.set(name, { type, name, price, amount: operator === Operator.ADD ? prev!.amount + 1 : prev!.amount - 1 })

      return cart
    })
  }

  const list = () => {
    const result: unknown[] = [];
    cart.forEach((v, k) => {
      result.push(
        <div class="flex justify-between">
          <div>{k}</div>
          <div class="inline-flex">
            <button class="px-3 py-1" onClick={() => handleChangeCartAmount(v, Operator.SUB)}>-</button>
            <div class="px-3 py-1">{v.amount}</div>
            <button class="px-3 py-1" onClick={() => handleChangeCartAmount(v, Operator.ADD)}>+</button>
          </div>
        </div>
      )
    })

    return result;
  }

  return (
    <div class="container absolute bottom-0 w-full bg-green-200 p-2">
      <div>{list()}</div>
      <div class="text-right">{`₩: ${mapLoop(cart)}`}</div>
    </div>
  )
}
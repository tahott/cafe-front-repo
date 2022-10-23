import { ComponentChildren, createContext } from "preact";
import { useContext, useEffect, useRef, useState } from "preact/hooks";
import { CartContext, Menu } from "./types.tsx";

export const useInterval = (callback: () => void, delay: number | null) => {
  const savedCallback = useRef(callback);

  useEffect(() => {
    if (delay !== null) {
      const id = setInterval(() => savedCallback.current(), delay);
      return () => clearInterval(id);
    }
  }, [delay]);
}

const CartCtx = createContext<CartContext>({ cart: [], setCart: () => { } });

export const CartProvider = ({ children }: { children: ComponentChildren }) => {
  const [cart, setCart] = useState<Array<Menu>>([]);

  return (
    <CartCtx.Provider value={{ cart, setCart }}>{children}</CartCtx.Provider>
  )
}

export const useCart = () => {
  const value = useContext(CartCtx);

  if (!value) {
    throw new Error('Cannot find Provider');
  }

  return value;
}

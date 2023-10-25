import { writable } from "svelte/store";

export const localStore = <T extends object>(key: string, initial?: T) => {
  const toString = (value: T) => JSON.stringify(value, null, 2);
  const toObj = JSON.parse;

  if (!localStorage.getItem(key) && initial) {
    // initialize local storage with initial value
    localStorage.setItem(key, toString(initial));
  }

  const saved = localStorage.getItem(key)
    ? toObj(localStorage.getItem(key)!)
    : undefined;

  const { subscribe, set, update } = writable(saved);

  return {
    subscribe,
    set: (value: T | undefined) => {
      if (value) {
        localStorage.setItem(key, toString(value));
        return set(value);
      }

      localStorage.removeItem(key)
    },
    update,
  };
};

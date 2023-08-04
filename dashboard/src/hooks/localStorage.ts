import { useCallback, useMemo } from "react";

export enum LocalStorage {
  Auth = "Auth",
}

export function useLocalStorage<T>(
  key: LocalStorage,
  parse: (value: unknown) => T,
) {
  const value = useMemo(() => {
    const value = localStorage.getItem(key);
    if (value) {
      return parse(JSON.parse(value));
    }
    return null;
  }, [key, parse]);

  const update = useCallback(
    (newValue: T | null) => {
      if (newValue === null) {
        localStorage.removeItem(key);
      } else {
        localStorage.setItem(key, JSON.stringify(parse(newValue)));
      }
    },
    [key, parse],
  );

  return {
    value,
    update,
  };
}

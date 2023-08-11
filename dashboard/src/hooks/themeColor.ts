import { useCallback } from "react";
import useSWR from "swr";

function getPropertyValue(name: string) {
  return getComputedStyle(document.body).getPropertyValue(name);
}

export function useThemeColor() {
  const themeFetcher = useCallback(() => {
    const primary = getPropertyValue("--p");
    const primaryContent = getPropertyValue("--pc");
    const secondary = getPropertyValue("--s");
    return { primary, primaryContent, secondary };
  }, []);

  const { data: color, mutate } = useSWR("data:theme", themeFetcher);

  return { ...color, mutate };
}

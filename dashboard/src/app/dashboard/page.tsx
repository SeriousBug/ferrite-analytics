"use client";
import { useAuth } from "@/hooks/auth";
import { endOfDay, format, formatRFC3339, startOfDay, subDays } from "date-fns";
import { useCallback, useMemo, useState } from "react";
import useSWRInfinite from "swr/infinite";
import { z } from "zod";
import { Line } from "react-chartjs-2";

import {
  Chart as ChartJS,
  LinearScale,
  CategoryScale,
  PointElement,
  LineElement,
} from "chart.js";
import { useThemeColor } from "@/hooks/themeColor";
import { LocalStorage, useLocalStorage } from "@/hooks/localStorage";
import { PiTrash } from "react-icons/pi";
import { API_BASE_URL } from "@/helpers/api";
ChartJS.register(CategoryScale, LinearScale, PointElement, LineElement);

type AnalyticsFilter =
  | { name: string; eq: string }
  | {
      and: AnalyticsFilter[];
    }
  | {
      or: AnalyticsFilter[];
    };

type AnalyticsRequest = {
  from_date: string;
  to_date: string;
  filter: AnalyticsFilter;
};

function useAnalyticsDataFetcher() {
  return useCallback(
    async ([token, { name, eq }, url, date]: [
      string,
      { name: string; eq: string },
      string,
      Date,
    ]) => {
      if (!token) throw new Error("No token");

      const body: AnalyticsRequest = {
        from_date: formatRFC3339(startOfDay(date)),
        to_date: formatRFC3339(endOfDay(date)),
        filter: {
          eq,
          name,
        },
      };

      const resp = await fetch(url, {
        headers: {
          Authorization: `Bearer ${token}`,
          "Content-Type": "application/json",
        },
        method: "POST",
        body: JSON.stringify(body),
      });
      return {
        label: format(date, "MMM dd"),
        value: z.coerce.number().parse(await resp.text()),
      };
    },
    [],
  );
}

function useAnalyticsData(name: string, eq: string) {
  const fetcher = useAnalyticsDataFetcher();
  const { token } = useAuth();
  const today = useMemo(() => new Date(), []);

  const getKey = useCallback(
    (pageIndex: number) => {
      return [
        token,
        { name, eq },
        `${API_BASE_URL}/query/filter`,
        subDays(today, pageIndex),
      ];
    },
    [eq, name, today, token],
  );

  const { data, error } = useSWRInfinite(getKey, fetcher, {
    initialSize: 14,
    revalidateOnFocus: false,
    // Cache results for 10 minutes
    dedupingInterval: 1000 * 60 * 10,
  });
  return {
    data: useMemo(() => data?.reverse(), [data]),
    error,
  };
}

function AnalyticsCard({
  name,
  eq,
  remove,
}: {
  name: string;
  eq: string;
  remove: () => void;
}) {
  const { primary, primaryContent } = useThemeColor();
  const { data } = useAnalyticsData(name, eq);

  const labels = useMemo(() => {
    return data?.map((d) => d.label);
  }, [data]);
  const values = useMemo(() => {
    return data?.map((d) => d.value);
  }, [data]);

  return (
    <div className="bg-base-100 p-8 rounded-box shadow-lg relative">
      <div className="relative h-[30vh]">
        <Line
          options={{ maintainAspectRatio: false }}
          data={{
            labels,
            datasets: [
              {
                data: values,
                // borderColor is the color of the line
                borderColor: `hsl(${primary})`,
                backgroundColor: `hsl(${primaryContent})`,
              },
            ],
          }}
        />
      </div>
      <button
        aria-label="delete"
        className="btn btn-circle absolute right-4 top-4"
        onClick={remove}
      >
        <PiTrash size={24} />
      </button>
    </div>
  );
}

const savedCardsSchema = z.object({
  cards: z.array(z.object({ name: z.string(), eq: z.string() })),
});

export default function Dashboard() {
  const [cards, setCards] = useLocalStorage(
    LocalStorage.Cards,
    savedCardsSchema.parse,
  );

  const [propertyName, setPropertyName] = useState("");
  const [propertyValue, setPropertyValue] = useState("");

  return (
    <>
      <div className="grid grid-cols-1 md:grid-cols-2 xl:grid-cols-3 gap-4">
        {cards
          ? cards.cards.map((card) => (
              <AnalyticsCard
                key={`${card.name}=${card.eq}`}
                name={card.name}
                eq={card.eq}
                remove={() => {
                  setCards({
                    cards: cards.cards.filter(
                      (c) => c.name !== card.name || c.eq !== card.eq,
                    ),
                  });
                }}
              />
            ))
          : null}
      </div>
      <div className="bg-base-100 p-8 rounded-box shadow-lg flex flex-col gap-2 max-w-sm mx-auto mt-8">
        <div>
          <label className="label" htmlFor="property-name">
            <span className="label-text">Property Name</span>
          </label>
          <input
            id="property-name"
            type="text"
            placeholder="tracking-pixel"
            className="input input-bordered w-full max-w-xs"
            onChange={(e) => setPropertyName(e.target.value)}
          />
        </div>
        <div>
          <label className="label" htmlFor="property-value">
            <span className="label-text">Property Value</span>
          </label>
          <input
            id="property-value"
            type="text"
            placeholder="tracking-pixel"
            className="input input-bordered w-full max-w-xs"
            onChange={(e) => setPropertyValue(e.target.value)}
          />
        </div>
        <button
          className="btn btn-secondary block max-w-xs mt-4"
          onClick={() => {
            setCards({
              cards: [
                ...(cards?.cards ?? []),
                {
                  name: propertyName,
                  eq: propertyValue,
                },
              ],
            });
          }}
        >
          Add
        </button>
      </div>
    </>
  );
}

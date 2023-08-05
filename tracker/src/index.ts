type TrackingEvent = {
  name: string;
  properties: Record<string, string | number | boolean | null>;
};
let queue: TrackingEvent[] = [];
let flushTimeout: number | undefined;

let sendEventsToUrl = "/t/event";

const QUEUE_FLUSH_LIMIT = 10;
const QUEUE_FLUSH_TIMEOUT = 1000;

/** Send the queued events right now.
 *
 * Clears the queue and cancels any pending flushes.
 */
const flush = async () => {
  navigator.sendBeacon(sendEventsToUrl, JSON.stringify(queue));
  queue = [];
  clearTimeout(flushTimeout);
  flushTimeout = undefined;
};

/** Send an event to be tracked.
 *
 * Events are not sent immediately, but are queued up and sent in batches.
 */
const ferrite = (
  name: TrackingEvent["name"],
  properties?: TrackingEvent["properties"],
) => {
  queue.push({
    name,
    properties: properties ?? {},
  });

  if (queue.length >= QUEUE_FLUSH_LIMIT) {
    // If queue length is reached, flush immediately
    flush();
  } else if (flushTimeout === undefined) {
    // Otherwise, wait to see if more events come in before flushing
    flushTimeout = setTimeout(flush, QUEUE_FLUSH_TIMEOUT);
  }
  // else, there's room in the queue and a flush is already scheduled, just wait
};
// @ts-ignore
window.ferrite = ferrite;

// Capture the page view
window.addEventListener("load", () => {
  ferrite("view", {
    path: window.location.pathname,
  });
});

// Send remaining events when the page is unloaded
document.addEventListener("visibilitychange", function logData() {
  if (document.visibilityState === "hidden") {
    flush();
  }
});

/** Load the provided configuration, setting up event listeners. */
const loadConfiguration = (configuration: Configuration) => {
  const { eventTrackers, visibilityTrackers, baseUrl } = configuration;

  if (baseUrl) {
    sendEventsToUrl = baseUrl + sendEventsToUrl;
  }

  eventTrackers?.forEach(({ selector, event, name }) => {
    document.querySelectorAll(selector).forEach((element) => {
      element.addEventListener(event, () => {
        ferrite(name ?? `${selector} ${event}`);
      });
    });
  });
  visibilityTrackers?.forEach(({ selector, name, ratioVisible = 100 }) => {
    const observer = new IntersectionObserver(
      () => {
        ferrite(name ?? `${selector} view`);
      },
      {
        root: null /* browser viewport */,
        threshold: ratioVisible / 100,
      },
    );
    document.querySelectorAll(selector).forEach((element) => {
      observer.observe(element);
    });
  });
};

type Configuration = {
  eventTrackers?: { selector: string; event: string; name?: string }[];
  visibilityTrackers?: {
    selector: string;
    name?: string;
    ratioVisible?: number;
  }[];
  baseUrl?: string;
};

const log = {
  error: (message: string, ...rest: unknown[]) => {
    console.error(`ferrite: ${message}`, ...rest);
  },
};

// Load the configuration from the script element, and load it.
//
// By loading the configuration from the script element, we can avoid having to
// make any additional requests, and the configuration is immediately available
// on load.
const scriptElement = document.currentScript;
if (scriptElement === null) {
  log.error("script element not found");
} else {
  try {
    const configuration = scriptElement.innerHTML;
    if (configuration.length > 0) {
      loadConfiguration(JSON.parse(configuration));
    }
  } catch (error) {
    log.error("failed to load configuration", error);
  }
}

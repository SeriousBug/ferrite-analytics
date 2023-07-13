import { nanoid } from "nanoid";

/** A temporary, random ID to identify a single user session.
 *
 * This information is stored in the session storage, so it will be cleared when
 * the website is closed.
 *
 * It's also per-website, so it can't track users across websites.
 */
const BASALYTICS_SESSION_STORAGE_KEY = "basalytics-session-id";
let sessionId: string | undefined;

type Event = {
  name: string;
  properties: Record<string, string | number | boolean | null>;
};
let queue: Event[] = [];
let flushTimeout: number | undefined;

const QUEUE_FLUSH_LIMIT = 10;
const QUEUE_FLUSH_TIMEOUT = 10000;

/** Send the queued events right now.
 *
 * Clears the queue and cancels any pending flushes.
 */
const flush = () => {
  fetch("http://localhost:3000/t/event", {
    method: "POST",
    body: JSON.stringify(queue),
    headers: {
      "Content-Type": "application/json",
    },
  });
  queue = [];
  clearTimeout(flushTimeout);
  flushTimeout = undefined;
};

let sampleRate = 1;

/** Send an event to be tracked.
 *
 * Events are not sent immediately, but are queued up and sent in batches.
 */
const basalytics = (name: Event["name"], properties?: Event["properties"]) => {
  if (Math.random() > sampleRate) {
    // If `sampleRate` is 0.1, then Math.random() will be greater than 0.1 90%
    // of the time, and thus 90% of the events will be dropped. 10% of the
    // events will be sent.
    return;
  }
  let props = properties ?? {};
  if (sessionId) {
    props = { ...props, sessionId };
  }
  queue.push({
    name,
    properties: props,
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
basalytics.sessionId = sessionId;

// @ts-ignore
window.basalytics = basalytics;

// Capture the page view
window.addEventListener("load", () => {
  basalytics("view", {
    path: window.location.pathname,
  });
});

/** Load the provided configuration, setting up event listeners. */
const loadConfiguration = (configuration: Configuration) => {
  const { eventTrackers, visibilityTrackers } = configuration;

  // We use a session ID, which is cleared when the website is closed, to track
  // user sessions. I don't think this counts as "data stored on users devices"
  // for ePrivacy Directive purposes, but we can let website owners decide that.
  if (configuration.trackSessions) {
    sessionId =
      window.sessionStorage.getItem(BASALYTICS_SESSION_STORAGE_KEY) ??
      undefined;
    if (sessionId === undefined) {
      sessionId = nanoid();
      window.sessionStorage.setItem(BASALYTICS_SESSION_STORAGE_KEY, sessionId);
    }
  }

  eventTrackers?.forEach(({ selector, event, name }) => {
    document.querySelectorAll(selector).forEach((element) => {
      element.addEventListener(event, () => {
        basalytics(name ?? `${selector} ${event}`);
      });
    });
  });
  visibilityTrackers?.forEach(({ selector, name, ratioVisible = 100 }) => {
    const observer = new IntersectionObserver(
      () => {
        basalytics(name ?? `${selector} view`);
      },
      {
        root: null /* browser viewport */,
        threshold: ratioVisible,
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
  sampleRate?: number;
  trackSessions?: boolean;
};

const log = {
  error: (message: string, ...rest: unknown[]) => {
    console.error(`basalytics: ${message}`, ...rest);
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

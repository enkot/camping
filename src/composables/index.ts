import { customRef } from "vue";
import { load, Store } from "@tauri-apps/plugin-store";

interface Options {
  onInit: (store: Store) => void;
}

export function useAsyncStorageRef<T>(
  key: string,
  initialValue: T,
  options?: Options
) {
  let store: Store;

  return customRef((track, trigger) => {
    let value: T = initialValue;

    // Initialize asynchronously
    load("store.json", { autoSave: false })
      .then((s) => {
        store = s;
        options?.onInit?.(s);
        return s.get<T>(key);
      })
      .then((stored) => {
        value = stored === undefined ? initialValue : stored;
        trigger();
      })
      .catch((err) => console.error("Error reading storage:", err));

    return {
      get() {
        track();
        return value;
      },
      async set(newValue) {
        value = newValue;
        trigger();

        try {
          await store.set(key, newValue);
        } catch (err) {
          console.error("Error writing to storage:", err);
        }
      },
    };
  });
}

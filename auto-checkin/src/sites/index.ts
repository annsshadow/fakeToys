import type { SiteAdapter } from "../types.js";
import { vyceaiAdapter } from "./vyceai.js";
import { agentrouterAdapter } from "./agentrouter.js";
import { justwokerAdapter } from "./justwoker.js";

/** 所有已注册的站点适配器 */
export const adapters: SiteAdapter[] = [
  vyceaiAdapter,
  agentrouterAdapter,
  justwokerAdapter,
];

export function getAdapter(siteId: string): SiteAdapter | undefined {
  return adapters.find((a) => a.id === siteId);
}

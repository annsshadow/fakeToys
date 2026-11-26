const origin = "http://127.0.0.1:5174";
await page.setViewportSize({ width: 390, height: 844 });
await page.goto(origin + "/", { waitUntil: "domcontentloaded" });
await page.evaluate(() => localStorage.clear());
await page.goto(origin + "/#/pages/skins/skins", { waitUntil: "networkidle" });
await page.waitForSelector(".rxn.skin", { timeout: 20000 });
const result = await page.evaluate(() => ({
  rows: document.querySelectorAll(".rxn.skin").length,
  names: [...document.querySelectorAll(".rxn.skin .rxn-name")].slice(0, 6).map(n => n.textContent.trim()),
  countText: document.querySelector(".result-count")?.textContent?.trim() || "",
  falseEmpty: document.body.innerText.includes("皮肤表为空（配置未加载？）") || document.body.innerText.includes("皮肤表还没加载出来"),
}));
await page.screenshot({ path: "docs/laiyipao/audits/round3-skins/screenshots/skins-deep-link-after.png", fullPage: false });
return result;

(() => {
  function copyButtons() {
    document.querySelectorAll(".codeblock").forEach((block) => {
      const button = block.querySelector(".codecopy");
      const code = block.querySelector("pre code");
      if (!button || !code) return;

      button.addEventListener("click", async () => {
        const text = code.innerText || code.textContent || "";
        try {
          await navigator.clipboard.writeText(text);
          const prev = button.textContent;
          button.textContent = "copied";
          button.classList.add("codecopy-done");
          setTimeout(() => {
            button.textContent = prev;
            button.classList.remove("codecopy-done");
          }, 1500);
        } catch (_) {
          const prev = button.textContent;
          button.textContent = "failed";
          setTimeout(() => {
            button.textContent = prev;
          }, 1500);
        }
      });
    });
  }

  function tocSpy() {
    const toc = document.getElementById("toc");
    if (!toc) return;
    const links = Array.from(toc.querySelectorAll(".toc-link"));
    if (!links.length) return;

    const targets = Array.from(document.querySelectorAll("h2[id], h3[id]"));
    if (!targets.length) return;

    const byId = new Map();
    links.forEach((l) => {
      const href = l.getAttribute("href") || "";
      if (!href.startsWith("#")) return;
      byId.set(href.slice(1), l);
    });

    const reset = () => {
      links.forEach((l) => l.classList.remove("active"));
    };

    const obs = new IntersectionObserver(
      (entries) => {
        entries.forEach((e) => {
          if (!e.isIntersecting) return;
          const id = e.target.getAttribute("id");
          if (!id) return;
          const link = byId.get(id);
          if (!link) return;
          reset();
          link.classList.add("active");
        });
      },
      { rootMargin: "-20% 0px -70% 0px", threshold: 0.01 }
    );

    targets.forEach((t) => obs.observe(t));
  }

  if (document.readyState === "loading") {
    document.addEventListener("DOMContentLoaded", () => {
      copyButtons();
      tocSpy();
    });
  } else {
    copyButtons();
    tocSpy();
  }
})();


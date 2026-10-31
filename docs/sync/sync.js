// Sync in Rapidkoi: the step-throughs. Each is a list of steps; Next animates into a step, Back shows it at rest.
(() => {
  'use strict';
  const reduce = window.matchMedia('(prefers-reduced-motion: reduce)').matches;
  const sleep = (ms) => new Promise((r) => setTimeout(r, reduce ? 0 : ms));
  const $ = (s, r = document) => r.querySelector(s);
  const $$ = (s, r = document) => Array.from(r.querySelectorAll(s));
  const LOCK = '<svg viewBox="0 0 20 20" aria-hidden="true"><rect x="3" y="9" width="14" height="10" rx="2" fill="currentColor"/><path d="M6 9V6a4 4 0 0 1 8 0v3" fill="none" stroke="currentColor" stroke-width="2.6"/></svg>';

  function stepper(root, steps, render, hold) {
    const prev = $('.st-prev', root), next = $('.st-next', root), play = $('.st-play', root);
    const count = $('.st-count', root), say = $('.st-say', root);
    let i = 0, token = 0, playing = false;
    async function show(n, animate) {
      i = n;
      const my = ++token;
      count.textContent = `Step ${i + 1} of ${steps.length}`;
      say.textContent = steps[i].say;
      prev.disabled = i === 0;
      next.disabled = i === steps.length - 1;
      await render(steps[i], i, animate && !reduce, () => my === token);
    }
    function stop() { playing = false; if (play) play.textContent = 'Play'; }
    async function run() {
      playing = true;
      play.textContent = 'Pause';
      if (i === steps.length - 1) await show(0, false);
      while (playing) {
        await sleep(hold || 2400);
        if (!playing) break;
        if (i === steps.length - 1) { stop(); break; }
        await show(i + 1, true);
      }
    }
    prev.addEventListener('click', () => { stop(); if (i > 0) show(i - 1, false); });
    next.addEventListener('click', () => { stop(); if (i < steps.length - 1) show(i + 1, true); });
    if (play) play.addEventListener('click', () => (playing ? stop() : run()));
    show(0, false);
  }

  // ---- The conversation: say what you have, send what's missing ----
  const DEV = ['A', 'B', 'C', 'P'];
  const ALL = ['A1', 'A2', 'A3', 'B1', 'B2', 'C1'];
  function summary(list) {
    const top = { A: 0, B: 0, C: 0 };
    for (const c of list) top[c[0]] = Math.max(top[c[0]], Number(c.slice(1)));
    return `A${top.A} B${top.B} C${top.C}`;
  }
  const EX = [
    { say: 'Each device makes its own changes, numbered: A1, A2, A3 on A, and so on. No device has another’s. The Pond is empty.',
      state: { A: ['A1', 'A2', 'A3'], B: ['B1', 'B2'], C: ['C1'], P: [] } },
    { say: 'Monday at home, no internet. A and B find each other on the Wi-Fi. Each says what it has, then sends what the other lacks.',
      link: 'AB', moves: [['A', 'B', ['A1', 'A2', 'A3']], ['B', 'A', ['B1', 'B2']]],
      state: { A: ['A1', 'A2', 'A3', 'B1', 'B2'], B: ['B1', 'B2', 'A1', 'A2', 'A3'], C: ['C1'], P: [] } },
    { say: 'Tuesday at work. C reaches the Pond over the internet. The Pond lacks C1, so C sends it. The Pond has nothing C lacks.',
      link: 'CP', moves: [['C', 'P', ['C1']]],
      state: { A: ['A1', 'A2', 'A3', 'B1', 'B2'], B: ['B1', 'B2', 'A1', 'A2', 'A3'], C: ['C1'], P: ['C1'] } },
    { say: 'Tuesday evening. A reaches the Pond. A sends its changes and B’s; the Pond sends C1. Everything travels sealed: the Pond can’t read any of it.',
      link: 'AP', moves: [['A', 'P', ['A1', 'A2', 'A3', 'B1', 'B2']], ['P', 'A', ['C1']]],
      state: { A: ALL, B: ['B1', 'B2', 'A1', 'A2', 'A3'], C: ['C1'], P: ['C1', 'A1', 'A2', 'A3', 'B1', 'B2'] } },
    { say: 'Wednesday. C reaches the Pond again and gets A’s and B’s changes. C never met B: B’s changes hopped B → A → Pond → C.',
      link: 'CP', moves: [['P', 'C', ['A1', 'A2', 'A3', 'B1', 'B2']]],
      state: { A: ALL, B: ['B1', 'B2', 'A1', 'A2', 'A3'], C: ['C1', 'A1', 'A2', 'A3', 'B1', 'B2'], P: ['C1', 'A1', 'A2', 'A3', 'B1', 'B2'] } },
    { say: 'Later, B meets A on the Wi-Fi again and gets C1. Everyone holds the same changes. The same three moves happened at every meeting.',
      link: 'AB', moves: [['A', 'B', ['C1']]],
      state: { A: ALL, B: ALL, C: ['C1', 'A1', 'A2', 'A3', 'B1', 'B2'], P: ['C1', 'A1', 'A2', 'A3', 'B1', 'B2'] } },
  ];
  function conversation(root) {
    const stage = $('.ex-stage', root), svg = $('svg', stage), packets = $('.ex-packets', stage);
    const node = (id) => $(`.ex-node[data-id="${id}"]`, stage);
    const center = (id) => {
      const s = stage.getBoundingClientRect(), r = node(id).getBoundingClientRect();
      return [r.left - s.left + r.width / 2, r.top - s.top + r.height / 2];
    };
    const chips = (id, list, fresh) => {
      $('.ex-chips', node(id)).innerHTML = list.length
        ? list.map((c) => `<span class="chip chip-${c[0]}${fresh === c ? ' new' : ''}">${c}</span>`).join('')
        : '<span class="ex-empty">nothing yet</span>';
    };
    const PAIRS = [['A', 'B', 'wifi'], ['A', 'P', 'net'], ['B', 'P', 'net'], ['C', 'P', 'net']];
    function lines(active) {
      const s = stage.getBoundingClientRect();
      svg.setAttribute('viewBox', `0 0 ${s.width} ${s.height}`);
      svg.innerHTML = PAIRS.map(([a, b, kind]) => {
        const [x1, y1] = center(a), [x2, y2] = center(b);
        const on = active && active.includes(a) && active.includes(b) ? ' on' : '';
        return `<line class="ex-line ${kind}${on}" x1="${x1}" y1="${y1}" x2="${x2}" y2="${y2}"/>`;
      }).join('');
    }
    function fly(from, to, label, delay, alive) {
      return sleep(delay).then(() => {
        if (!alive()) return;
        const [x0, y0] = center(from), [x1, y1] = center(to);
        const p = document.createElement('span');
        p.className = `packet chip-${label[0]}`;
        p.innerHTML = LOCK + label;
        packets.append(p);
        const a = p.animate([
          { transform: `translate(${x0}px, ${y0}px) translate(-50%, -50%)` },
          { transform: `translate(${x1}px, ${y1}px) translate(-50%, -50%)` },
        ], { duration: 950, easing: 'cubic-bezier(.45, 0, .25, 1)', fill: 'forwards' });
        return a.finished.then(() => p.remove(), () => p.remove());
      });
    }
    let current = 0;
    async function render(step, idx, animate, alive) {
      current = idx;
      packets.innerHTML = '';
      $$('.ex-bubble', stage).forEach((b) => b.classList.remove('show'));
      lines(step.link);
      if (!animate || !step.moves) { for (const id of DEV) chips(id, step.state[id]); return; }
      const before = EX[idx - 1].state;
      const have = {};
      for (const id of DEV) { have[id] = before[id].slice(); chips(id, have[id]); }
      for (const id of step.link) {
        const b = $('.ex-bubble', node(id));
        b.textContent = `I have ${summary(before[id])}`;
        b.classList.add('show');
      }
      await sleep(1500);
      if (!alive()) return;
      const flights = [];
      let k = 0;
      for (const [from, to, items] of step.moves) {
        for (const c of items) {
          flights.push(fly(from, to, c, k++ * 170, alive).then(() => {
            if (!alive()) return;
            have[to].push(c);
            chips(to, have[to], c);
          }));
        }
      }
      await Promise.all(flights);
      await sleep(600);
      if (!alive()) return;
      $$('.ex-bubble', stage).forEach((b) => b.classList.remove('show'));
      for (const id of DEV) chips(id, step.state[id]);
    }
    window.addEventListener('resize', () => lines(EX[current].link));
    stepper(root, EX, render, 2600);
  }

  // ---- Merging, field by field ----
  const F = (v, t) => ({ v, t });
  const MG = [
    { say: 'All three devices hold task 7, made on A at 09:00 (change A1).',
      rows: { A: [F('"Milk"', '09:00 A'), F('no', '09:00 A'), ['A1']], B: [F('"Milk"', '09:00 A'), F('no', '09:00 A'), ['A1']], C: [F('"Milk"', '09:00 A'), F('no', '09:00 A'), ['A1']] } },
    { say: 'Apart, each edits it. A renames it at 10:03 (A2). B renames it at 10:02 and ticks it at 10:02 (B1, B2). C renames it at 09:30 (C1). Three different screens.',
      hits: ['A0', 'B0', 'B1', 'C0'],
      rows: { A: [F('"Buy milk"', '10:03 A'), F('no', '09:00 A'), ['A1', 'A2']], B: [F('"Oat milk"', '10:02 B'), F('yes', '10:02 B'), ['A1', 'B1', 'B2']], C: [F('"Milk 2L"', '09:30 C'), F('no', '09:00 A'), ['A1', 'C1']] } },
    { say: 'A and B swap. Title: A’s 10:03 beats B’s 10:02, on both. Done: only B changed it, so B’s tick wins on both. B1 lost, but it stays in the log.',
      hits: ['A1', 'B0'], lost: ['B1'],
      rows: { A: [F('"Buy milk"', '10:03 A'), F('yes', '10:02 B'), ['A1', 'A2', 'B1', 'B2']], B: [F('"Buy milk"', '10:03 A'), F('yes', '10:02 B'), ['A1', 'B1', 'B2', 'A2']], C: [F('"Milk 2L"', '09:30 C'), F('no', '09:00 A'), ['A1', 'C1']] } },
    { say: 'C meets A. C takes A2 (10:03 beats its 09:30), ignores B1 (10:02 is older than 10:03 now) and takes B2. A ignores C1. Order of arrival made no difference: all three match.',
      hits: ['C0', 'C1'], lost: ['B1', 'C1'],
      rows: { A: [F('"Buy milk"', '10:03 A'), F('yes', '10:02 B'), ['A1', 'A2', 'B1', 'B2', 'C1']], B: [F('"Buy milk"', '10:03 A'), F('yes', '10:02 B'), ['A1', 'B1', 'B2', 'A2']], C: [F('"Buy milk"', '10:03 A'), F('yes', '10:02 B'), ['A1', 'C1', 'A2', 'B1', 'B2']] } },
  ];
  function merging(root) {
    const body = $('tbody', root);
    stepper(root, MG, (step, idx, animate) => {
      body.innerHTML = ['A', 'B', 'C'].map((d) => {
        const [title, done, log] = step.rows[d];
        const cell = (f, n) => `<td class="${animate && (step.hits || []).includes(d + n) ? 'hit' : ''}"><span class="v">${f.v}</span><span class="t">${f.t}</span></td>`;
        const logs = log.map((c) => `<span class="chip chip-${c[0]}${(step.lost || []).includes(c) ? ' lost' : ''}">${c}</span>`).join('');
        return `<tr><th scope="row">Device ${d}</th>${cell(title, 0)}${cell(done, 1)}<td><span class="logs">${logs}</span></td></tr>`;
      }).join('');
    }, 3200);
  }

  // ---- The clock ----
  const CK = [
    { say: 'The real time is 10:00. A’s clock is right; C’s is two hours slow.',
      cards: [['A', '10:00', '—', '"Milk"  (09:00)'], ['C', '08:00', '—', '"Milk"  (09:00)']] },
    { say: 'A renames the task at 10:00, and C receives it.', hits: ['A3', 'C3'],
      cards: [['A', '10:00', '10:00', '"Buy milk"  (10:00)'], ['C', '08:00', '—', '"Buy milk"  (10:00)']] },
    { say: 'Without the fix: five minutes later the person corrects the title on C, which stamps its slow clock, 08:05.', hits: ['C3'],
      cards: [['A', '10:05', '10:00', '"Buy milk"  (10:00)'], ['C', '08:05', '—', '"Buy oat milk"  (08:05)']] },
    { say: 'They swap: 10:00 beats 08:05, so the correction loses everywhere, though it came last.', hits: ['C3'], verdict: [false, 'The newer edit lost.'],
      cards: [['A', '10:05', '10:00', '"Buy milk"  (10:00)'], ['C', '08:05', '10:00', '"Buy milk"  (10:00)']] },
    { say: 'With the fix: when C received A’s change it remembered the latest time it had seen, 10:00.', hits: ['C2'],
      cards: [['A', '10:00', '10:00', '"Buy milk"  (10:00)'], ['C', '08:00', '10:00', '"Buy milk"  (10:00)']] },
    { say: 'C’s correction takes the later of its clock (08:05) and the latest seen (10:00), plus a tick.', hits: ['C3'],
      cards: [['A', '10:05', '10:00', '"Buy milk"  (10:00)'], ['C', '08:05', '10:00 +1', '"Buy oat milk"  (10:00 +1)']] },
    { say: 'They swap: 10:00 +1 beats 10:00. An edit made after seeing another always counts as later, whatever the clock says.', hits: ['A3'], verdict: [true, 'The newer edit won.'],
      cards: [['A', '10:05', '10:00 +1', '"Buy oat milk"  (10:00 +1)'], ['C', '08:05', '10:00 +1', '"Buy oat milk"  (10:00 +1)']] },
  ];
  function clock(root) {
    const out = $('.ck', root), verdict = $('.verdict', root);
    stepper(root, CK, (step, idx, animate) => {
      const hit = (k) => (animate && (step.hits || []).includes(k) ? ' class="v hit"' : ' class="v"');
      out.innerHTML = step.cards.map(([d, clk, seen, title]) => `<div class="ck-card"><h4>Device ${d}</h4>` +
        `<span class="k">Its clock</span><span${hit(d + 1)}>${clk}</span><span class="k">Latest seen</span><span${hit(d + 2)}>${seen}</span>` +
        `<span class="k">Title</span><span${hit(d + 3)}>${title}</span></div>`).join('');
      verdict.className = 'verdict' + (step.verdict ? (step.verdict[0] ? ' ok' : ' bad') : '');
      verdict.textContent = step.verdict ? step.verdict[1] : '';
    }, 3000);
  }

  // ---- Two devices meeting on Wi-Fi ----
  const WF = [
    { say: '1. Listen. With sync on, each device opens a port on the local network and waits for its own other devices.' },
    { say: '2. Announce. A says "I’m here" to the whole network. It carries only a tag from the person key: random-looking to strangers, changing over time.' },
    { say: '3. Connect. B recognises the tag and opens a direct connection to A.' },
    { say: '4. Prove. Each shows it holds a device key on the person’s signed device list, and the connection is encrypted (mutual TLS). Anything wrong: it closes, nothing sent.' },
    { say: '5. Exchange. Say what you have, send what’s missing, merge. While both stay on the network, each new change goes across as it’s made.' },
  ];
  function wifi(root) {
    stepper(root, WF, (step, idx) => {
      for (const el of $$('[data-on]', root)) {
        const [a, b] = el.dataset.on.split('-').map(Number);
        el.style.opacity = idx + 1 >= a && idx + 1 <= (b || 9) ? 1 : 0;
      }
    }, 3600);
  }

  conversation($('#st-conversation'));
  merging($('#st-merge'));
  clock($('#st-clock'));
  wifi($('#st-wifi'));
})();

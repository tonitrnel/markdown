const __vite__mapDeps=(i,m=__vite__mapDeps,d=(m.f||(m.f=["assets/angular-html-LfdN0zeE.js","assets/html-C2L_23MC.js","assets/javascript-ySlJ1b_l.js","assets/css-BPhBrDlE.js","assets/angular-ts-CKsD7JZE.js","assets/scss-C31hgJw-.js","assets/apl-BBq3IX1j.js","assets/xml-e3z08dGr.js","assets/java-xI-RfyKK.js","assets/json-BQoSv7ci.js","assets/astro-CqkE3fuf.js","assets/typescript-Dj6nwHGl.js","assets/postcss-B3ZDOciz.js","assets/blade-a8OxSdnT.js","assets/sql-COK4E0Yg.js","assets/bsl-Dgyn0ogV.js","assets/sdbl-BLhTXw86.js","assets/cairo--RitsXJZ.js","assets/python-DhUJRlN_.js","assets/cobol-PTqiYgYu.js","assets/coffee-dyiR41kL.js","assets/cpp-BksuvNSY.js","assets/regexp-DWJ3fJO_.js","assets/glsl-DBO2IWDn.js","assets/c-C3t2pwGQ.js","assets/crystal-DtDmRg-F.js","assets/shellscript-atvbtKCR.js","assets/edge-D5gP-w-T.js","assets/html-derivative-CSfWNPLT.js","assets/elixir-CLiX3zqd.js","assets/elm-CmHSxxaM.js","assets/erb-BYTLMnw6.js","assets/ruby-DeZ3UC14.js","assets/haml-B2EZWmdv.js","assets/graphql-cDcHW_If.js","assets/jsx-BAng5TT0.js","assets/tsx-B6W0miNI.js","assets/lua-CvWAzNxB.js","assets/yaml-CVw76BM1.js","assets/fortran-fixed-form-TqA4NnZg.js","assets/fortran-free-form-DKXYxT9g.js","assets/fsharp-XplgxFYe.js","assets/markdown-UIAJJxZW.js","assets/gdresource-BHYsBjWJ.js","assets/gdshader-SKMF96pI.js","assets/gdscript-DfxzS6Rs.js","assets/git-commit-i4q6IMui.js","assets/diff-BgYniUM_.js","assets/git-rebase-B-v9cOL2.js","assets/glimmer-js-D-cwc0-E.js","assets/glimmer-ts-pgjy16dm.js","assets/hack-D1yCygmZ.js","assets/handlebars-BQGss363.js","assets/http-FRrOvY1W.js","assets/hxml-TIA70rKU.js","assets/haxe-C5wWYbrZ.js","assets/imba-bv_oIlVt.js","assets/jinja-DGy0s7-h.js","assets/jison-BqZprYcd.js","assets/julia-BBuGR-5E.js","assets/r-CwjWoCRV.js","assets/latex-C-cWTeAZ.js","assets/tex-rYs2v40G.js","assets/liquid-D3W5UaiH.js","assets/marko-z0MBrx5-.js","assets/less-BfCpw3nA.js","assets/mdc-DB_EDNY_.js","assets/nginx-D_VnBJ67.js","assets/nim-ZlGxZxc3.js","assets/perl-CHQXSrWU.js","assets/php-B5ebYQev.js","assets/pug-CM9l7STV.js","assets/qml-D8XfuvdV.js","assets/razor-CNLDkMZG.js","assets/csharp-D9R-vmeu.js","assets/rst-4NLicBqY.js","assets/cmake-DbXoA79R.js","assets/sas-BmTFh92c.js","assets/shaderlab-B7qAK45m.js","assets/hlsl-ifBTmRxC.js","assets/shellsession-C_rIy8kc.js","assets/soy-C-lX7w71.js","assets/sparql-bYkjHRlG.js","assets/turtle-BMR_PYu6.js","assets/stata-DorPZHa4.js","assets/svelte-MSaWC3Je.js","assets/templ-dwX3ZSMB.js","assets/go-B1SYOhNW.js","assets/ts-tags-CipyTH0X.js","assets/twig-NC5TFiHP.js","assets/vue-BuYVFjOK.js","assets/vue-html-xdeiXROB.js","assets/xsl-Dd0NUgwM.js"])))=>i.map(i=>d[i]);
(async ()=>{
    (function() {
        const e = document.createElement("link").relList;
        if (e && e.supports && e.supports("modulepreload")) return;
        for (const i of document.querySelectorAll('link[rel="modulepreload"]'))r(i);
        new MutationObserver((i)=>{
            for (const o of i)if (o.type === "childList") for (const s of o.addedNodes)s.tagName === "LINK" && s.rel === "modulepreload" && r(s);
        }).observe(document, {
            childList: !0,
            subtree: !0
        });
        function n(i) {
            const o = {};
            return i.integrity && (o.integrity = i.integrity), i.referrerPolicy && (o.referrerPolicy = i.referrerPolicy), i.crossOrigin === "use-credentials" ? o.credentials = "include" : i.crossOrigin === "anonymous" ? o.credentials = "omit" : o.credentials = "same-origin", o;
        }
        function r(i) {
            if (i.ep) return;
            i.ep = !0;
            const o = n(i);
            fetch(i.href, o);
        }
    })();
    const oi = !1, si = (t, e)=>t === e, ai = Symbol("solid-track"), dt = {
        equals: si
    };
    let er = or;
    const he = 1, _t = 2, tr = {
        owned: null,
        cleanups: null,
        context: null,
        owner: null
    };
    var M = null;
    let Nt = null, li = null, x = null, F = null, le = null, Rt = 0;
    function at(t, e) {
        const n = x, r = M, i = t.length === 0, o = e === void 0 ? r : e, s = i ? tr : {
            owned: null,
            cleanups: null,
            context: o ? o.context : null,
            owner: o
        }, a = i ? t : ()=>t(()=>be(()=>Ue(s)));
        M = s, x = null;
        try {
            return Ke(a, !0);
        } finally{
            x = n, M = r;
        }
    }
    function te(t, e) {
        e = e ? Object.assign({}, dt, e) : dt;
        const n = {
            value: t,
            observers: null,
            observerSlots: null,
            comparator: e.equals || void 0
        }, r = (i)=>(typeof i == "function" && (i = i(n.value)), ir(n, i));
        return [
            rr.bind(n),
            r
        ];
    }
    function ie(t, e, n) {
        const r = an(t, e, !1, he);
        ze(r);
    }
    function nr(t, e, n) {
        er = _i;
        const r = an(t, e, !1, he);
        r.user = !0, le ? le.push(r) : ze(r);
    }
    function Ae(t, e, n) {
        n = n ? Object.assign({}, dt, n) : dt;
        const r = an(t, e, !0, 0);
        return r.observers = null, r.observerSlots = null, r.comparator = n.equals || void 0, ze(r), rr.bind(r);
    }
    function be(t) {
        if (x === null) return t();
        const e = x;
        x = null;
        try {
            return t();
        } finally{
            x = e;
        }
    }
    function ci(t) {
        return M === null || (M.cleanups === null ? M.cleanups = [
            t
        ] : M.cleanups.push(t)), t;
    }
    function rr() {
        if (this.sources && this.state) if (this.state === he) ze(this);
        else {
            const t = F;
            F = null, Ke(()=>ft(this), !1), F = t;
        }
        if (x) {
            const t = this.observers;
            if (!t || t[t.length - 1] !== x) {
                const e = t ? t.length : 0;
                x.sources ? (x.sources.push(this), x.sourceSlots.push(e)) : (x.sources = [
                    this
                ], x.sourceSlots = [
                    e
                ]), t ? (t.push(x), this.observerSlots.push(x.sources.length - 1)) : (this.observers = [
                    x
                ], this.observerSlots = [
                    x.sources.length - 1
                ]);
            }
        }
        return this.value;
    }
    function ir(t, e, n) {
        let r = t.value;
        return (!t.comparator || !t.comparator(r, e)) && (t.value = e, t.observers && t.observers.length && Ke(()=>{
            for(let i = 0; i < t.observers.length; i += 1){
                const o = t.observers[i], s = Nt && Nt.running;
                s && Nt.disposed.has(o), (s ? !o.tState : !o.state) && (o.pure ? F.push(o) : le.push(o), o.observers && sr(o)), s || (o.state = he);
            }
            if (F.length > 1e6) throw F = [], new Error;
        }, !1)), e;
    }
    function ze(t) {
        if (!t.fn) return;
        Ue(t);
        const e = Rt;
        ui(t, t.value, e);
    }
    function ui(t, e, n) {
        let r;
        const i = M, o = x;
        x = M = t;
        try {
            r = t.fn(e);
        } catch (s) {
            return t.pure && (t.state = he, t.owned && t.owned.forEach(Ue), t.owned = null), t.updatedAt = n + 1, ar(s);
        } finally{
            x = o, M = i;
        }
        (!t.updatedAt || t.updatedAt <= n) && (t.updatedAt != null && "observers" in t ? ir(t, r) : t.value = r, t.updatedAt = n);
    }
    function an(t, e, n, r = he, i) {
        const o = {
            fn: t,
            state: r,
            updatedAt: null,
            owned: null,
            sources: null,
            sourceSlots: null,
            cleanups: null,
            value: e,
            owner: M,
            context: M ? M.context : null,
            pure: n
        };
        return M === null || M !== tr && (M.owned ? M.owned.push(o) : M.owned = [
            o
        ]), o;
    }
    function ht(t) {
        if (t.state === 0) return;
        if (t.state === _t) return ft(t);
        if (t.suspense && be(t.suspense.inFallback)) return t.suspense.effects.push(t);
        const e = [
            t
        ];
        for(; (t = t.owner) && (!t.updatedAt || t.updatedAt < Rt);)t.state && e.push(t);
        for(let n = e.length - 1; n >= 0; n--)if (t = e[n], t.state === he) ze(t);
        else if (t.state === _t) {
            const r = F;
            F = null, Ke(()=>ft(t, e[0]), !1), F = r;
        }
    }
    function Ke(t, e) {
        if (F) return t();
        let n = !1;
        e || (F = []), le ? n = !0 : le = [], Rt++;
        try {
            const r = t();
            return di(n), r;
        } catch (r) {
            n || (le = null), F = null, ar(r);
        }
    }
    function di(t) {
        if (F && (or(F), F = null), t) return;
        const e = le;
        le = null, e.length && Ke(()=>er(e), !1);
    }
    function or(t) {
        for(let e = 0; e < t.length; e++)ht(t[e]);
    }
    function _i(t) {
        let e, n = 0;
        for(e = 0; e < t.length; e++){
            const r = t[e];
            r.user ? t[n++] = r : ht(r);
        }
        for(e = 0; e < n; e++)ht(t[e]);
    }
    function ft(t, e) {
        t.state = 0;
        for(let n = 0; n < t.sources.length; n += 1){
            const r = t.sources[n];
            if (r.sources) {
                const i = r.state;
                i === he ? r !== e && (!r.updatedAt || r.updatedAt < Rt) && ht(r) : i === _t && ft(r, e);
            }
        }
    }
    function sr(t) {
        for(let e = 0; e < t.observers.length; e += 1){
            const n = t.observers[e];
            n.state || (n.state = _t, n.pure ? F.push(n) : le.push(n), n.observers && sr(n));
        }
    }
    function Ue(t) {
        let e;
        if (t.sources) for(; t.sources.length;){
            const n = t.sources.pop(), r = t.sourceSlots.pop(), i = n.observers;
            if (i && i.length) {
                const o = i.pop(), s = n.observerSlots.pop();
                r < i.length && (o.sourceSlots[s] = r, i[r] = o, n.observerSlots[r] = s);
            }
        }
        if (t.tOwned) {
            for(e = t.tOwned.length - 1; e >= 0; e--)Ue(t.tOwned[e]);
            delete t.tOwned;
        }
        if (t.owned) {
            for(e = t.owned.length - 1; e >= 0; e--)Ue(t.owned[e]);
            t.owned = null;
        }
        if (t.cleanups) {
            for(e = t.cleanups.length - 1; e >= 0; e--)t.cleanups[e]();
            t.cleanups = null;
        }
        t.state = 0;
    }
    function hi(t) {
        return t instanceof Error ? t : new Error(typeof t == "string" ? t : "Unknown error", {
            cause: t
        });
    }
    function ar(t, e = M) {
        throw hi(t);
    }
    const fi = Symbol("fallback");
    function Sn(t) {
        for(let e = 0; e < t.length; e++)t[e]();
    }
    function pi(t, e, n = {}) {
        let r = [], i = [], o = [], s = 0, a = e.length > 1 ? [] : null;
        return ci(()=>Sn(o)), ()=>{
            let l = t() || [], c = l.length, d, _;
            return l[ai], be(()=>{
                let f, h, E, g, w, m, y, v, R;
                if (c === 0) s !== 0 && (Sn(o), o = [], r = [], i = [], s = 0, a && (a = [])), n.fallback && (r = [
                    fi
                ], i[0] = at((O)=>(o[0] = O, n.fallback())), s = 1);
                else if (s === 0) {
                    for(i = new Array(c), _ = 0; _ < c; _++)r[_] = l[_], i[_] = at(p);
                    s = c;
                } else {
                    for(E = new Array(c), g = new Array(c), a && (w = new Array(c)), m = 0, y = Math.min(s, c); m < y && r[m] === l[m]; m++);
                    for(y = s - 1, v = c - 1; y >= m && v >= m && r[y] === l[v]; y--, v--)E[v] = i[y], g[v] = o[y], a && (w[v] = a[y]);
                    for(f = new Map, h = new Array(v + 1), _ = v; _ >= m; _--)R = l[_], d = f.get(R), h[_] = d === void 0 ? -1 : d, f.set(R, _);
                    for(d = m; d <= y; d++)R = r[d], _ = f.get(R), _ !== void 0 && _ !== -1 ? (E[_] = i[d], g[_] = o[d], a && (w[_] = a[d]), _ = h[_], f.set(R, _)) : o[d]();
                    for(_ = m; _ < c; _++)_ in E ? (i[_] = E[_], o[_] = g[_], a && (a[_] = w[_], a[_](_))) : i[_] = at(p);
                    i = i.slice(0, s = c), r = l.slice(0);
                }
                return i;
            });
            function p(f) {
                if (o[_] = f, a) {
                    const [h, E] = te(_);
                    return a[_] = E, e(l[_], h);
                }
                return e(l[_]);
            }
        };
    }
    function I(t, e) {
        return be(()=>t(e || {}));
    }
    const mi = (t)=>`Stale read from <${t}>.`;
    function lr(t) {
        const e = "fallback" in t && {
            fallback: ()=>t.fallback
        };
        return Ae(pi(()=>t.each, t.children, e || void 0));
    }
    function U(t) {
        const e = t.keyed, n = Ae(()=>t.when, void 0, void 0), r = e ? n : Ae(n, void 0, {
            equals: (i, o)=>!i == !o
        });
        return Ae(()=>{
            const i = r();
            if (i) {
                const o = t.children;
                return typeof o == "function" && o.length > 0 ? be(()=>o(e ? i : ()=>{
                        if (!be(r)) throw mi("Show");
                        return n();
                    })) : o;
            }
            return t.fallback;
        }, void 0, void 0);
    }
    const Me = (t)=>Ae(()=>t());
    function gi(t, e, n) {
        let r = n.length, i = e.length, o = r, s = 0, a = 0, l = e[i - 1].nextSibling, c = null;
        for(; s < i || a < o;){
            if (e[s] === n[a]) {
                s++, a++;
                continue;
            }
            for(; e[i - 1] === n[o - 1];)i--, o--;
            if (i === s) {
                const d = o < r ? a ? n[a - 1].nextSibling : n[o - a] : l;
                for(; a < o;)t.insertBefore(n[a++], d);
            } else if (o === a) for(; s < i;)(!c || !c.has(e[s])) && e[s].remove(), s++;
            else if (e[s] === n[o - 1] && n[a] === e[i - 1]) {
                const d = e[--i].nextSibling;
                t.insertBefore(n[a++], e[s++].nextSibling), t.insertBefore(n[--o], d), e[i] = n[o];
            } else {
                if (!c) {
                    c = new Map;
                    let _ = a;
                    for(; _ < o;)c.set(n[_], _++);
                }
                const d = c.get(e[s]);
                if (d != null) if (a < d && d < o) {
                    let _ = s, p = 1, f;
                    for(; ++_ < i && _ < o && !((f = c.get(e[_])) == null || f !== d + p);)p++;
                    if (p > d - a) {
                        const h = e[s];
                        for(; a < d;)t.insertBefore(n[a++], h);
                    } else t.replaceChild(n[a++], e[s++]);
                } else s++;
                else e[s++].remove();
            }
        }
    }
    const An = "_$DX_DELEGATE";
    function bi(t, e, n, r = {}) {
        let i;
        return at((o)=>{
            i = o, e === document ? t() : L(e, t(), e.firstChild ? null : void 0, n);
        }, r.owner), ()=>{
            i(), e.textContent = "";
        };
    }
    function N(t, e, n, r) {
        let i;
        const o = ()=>{
            const a = document.createElement("template");
            return a.innerHTML = t, a.content.firstChild;
        }, s = ()=>(i || (i = o())).cloneNode(!0);
        return s.cloneNode = s, s;
    }
    function cr(t, e = window.document) {
        const n = e[An] || (e[An] = new Set);
        for(let r = 0, i = t.length; r < i; r++){
            const o = t[r];
            n.has(o) || (n.add(o), e.addEventListener(o, Ei));
        }
    }
    function Tn(t, e, n) {
        n == null ? t.removeAttribute(e) : t.setAttribute(e, n);
    }
    function Ne(t, e) {
        e == null ? t.removeAttribute("class") : t.className = e;
    }
    function yi(t, e, n, r) {
        if (Array.isArray(n)) {
            const i = n[0];
            t.addEventListener(e, n[0] = (o)=>i.call(t, n[1], o));
        } else t.addEventListener(e, n, typeof n != "function" && n);
    }
    function wi(t, e, n) {
        return be(()=>t(e, n));
    }
    function L(t, e, n, r) {
        if (n !== void 0 && !r && (r = []), typeof e != "function") return pt(t, e, r, n);
        ie((i)=>pt(t, e(), i, n), r);
    }
    function Ei(t) {
        let e = t.target;
        const n = `$$${t.type}`, r = t.target, i = t.currentTarget, o = (l)=>Object.defineProperty(t, "target", {
                configurable: !0,
                value: l
            }), s = ()=>{
            const l = e[n];
            if (l && !e.disabled) {
                const c = e[`${n}Data`];
                if (c !== void 0 ? l.call(e, c, t) : l.call(e, t), t.cancelBubble) return;
            }
            return e.host && typeof e.host != "string" && !e.host._$host && e.contains(t.target) && o(e.host), !0;
        }, a = ()=>{
            for(; s() && (e = e._$host || e.parentNode || e.host););
        };
        if (Object.defineProperty(t, "currentTarget", {
            configurable: !0,
            get () {
                return e || document;
            }
        }), t.composedPath) {
            const l = t.composedPath();
            o(l[0]);
            for(let c = 0; c < l.length - 2 && (e = l[c], !!s()); c++){
                if (e._$host) {
                    e = e._$host, a();
                    break;
                }
                if (e.parentNode === i) break;
            }
        } else a();
        o(r);
    }
    function pt(t, e, n, r, i) {
        for(; typeof n == "function";)n = n();
        if (e === n) return n;
        const o = typeof e, s = r !== void 0;
        if (t = s && n[0] && n[0].parentNode || t, o === "string" || o === "number") {
            if (o === "number" && (e = e.toString(), e === n)) return n;
            if (s) {
                let a = n[0];
                a && a.nodeType === 3 ? a.data !== e && (a.data = e) : a = document.createTextNode(e), n = ve(t, n, r, a);
            } else n !== "" && typeof n == "string" ? n = t.firstChild.data = e : n = t.textContent = e;
        } else if (e == null || o === "boolean") n = ve(t, n, r);
        else {
            if (o === "function") return ie(()=>{
                let a = e();
                for(; typeof a == "function";)a = a();
                n = pt(t, a, n, r);
            }), ()=>n;
            if (Array.isArray(e)) {
                const a = [], l = n && Array.isArray(n);
                if (qt(a, e, n, i)) return ie(()=>n = pt(t, a, n, r, !0)), ()=>n;
                if (a.length === 0) {
                    if (n = ve(t, n, r), s) return n;
                } else l ? n.length === 0 ? Rn(t, a, r) : gi(t, n, a) : (n && ve(t), Rn(t, a));
                n = a;
            } else if (e.nodeType) {
                if (Array.isArray(n)) {
                    if (s) return n = ve(t, n, r, e);
                    ve(t, n, null, e);
                } else n == null || n === "" || !t.firstChild ? t.appendChild(e) : t.replaceChild(e, t.firstChild);
                n = e;
            }
        }
        return n;
    }
    function qt(t, e, n, r) {
        let i = !1;
        for(let o = 0, s = e.length; o < s; o++){
            let a = e[o], l = n && n[t.length], c;
            if (!(a == null || a === !0 || a === !1)) if ((c = typeof a) == "object" && a.nodeType) t.push(a);
            else if (Array.isArray(a)) i = qt(t, a, l) || i;
            else if (c === "function") if (r) {
                for(; typeof a == "function";)a = a();
                i = qt(t, Array.isArray(a) ? a : [
                    a
                ], Array.isArray(l) ? l : [
                    l
                ]) || i;
            } else t.push(a), i = !0;
            else {
                const d = String(a);
                l && l.nodeType === 3 && l.data === d ? t.push(l) : t.push(document.createTextNode(d));
            }
        }
        return i;
    }
    function Rn(t, e, n = null) {
        for(let r = 0, i = e.length; r < i; r++)t.insertBefore(e[r], n);
    }
    function ve(t, e, n, r) {
        if (n === void 0) return t.textContent = "";
        const i = r || document.createTextNode("");
        if (e.length) {
            let o = !1;
            for(let s = e.length - 1; s >= 0; s--){
                const a = e[s];
                if (i !== a) {
                    const l = a.parentNode === t;
                    !o && !s ? l ? t.replaceChild(i, a) : t.insertBefore(i, n) : l && a.remove();
                } else o = !0;
            }
        } else t.insertBefore(i, n);
        return [
            i
        ];
    }
    const vi = "/markdown/assets/markdown_binding_bg-BYAOKboP.wasm", Si = async (t = {}, e)=>{
        let n;
        if (e.startsWith("data:")) {
            const r = e.replace(/^data:.*?base64,/, "");
            let i;
            if (typeof Buffer == "function" && typeof Buffer.from == "function") i = Buffer.from(r, "base64");
            else if (typeof atob == "function") {
                const o = atob(r);
                i = new Uint8Array(o.length);
                for(let s = 0; s < o.length; s++)i[s] = o.charCodeAt(s);
            } else throw new Error("Cannot decode base64-encoded data URL");
            n = await WebAssembly.instantiate(i, t);
        } else {
            const r = await fetch(e), i = r.headers.get("Content-Type") || "";
            if ("instantiateStreaming" in WebAssembly && i.startsWith("application/wasm")) n = await WebAssembly.instantiateStreaming(r, t);
            else {
                const o = await r.arrayBuffer();
                n = await WebAssembly.instantiate(o, t);
            }
        }
        return n.instance.exports;
    };
    let Le = class ur {
        static __wrap(e) {
            const n = Object.create(ur.prototype);
            return n.__wbg_ptr = e, Ln.register(n, n.__wbg_ptr, n), n;
        }
        __destroy_into_raw() {
            const e = this.__wbg_ptr;
            return this.__wbg_ptr = 0, Ln.unregister(this), e;
        }
        free() {
            const e = this.__destroy_into_raw();
            S.__wbg_document_free(e, 0);
        }
        astData() {
            const e = S.document_astData(this.__wbg_ptr);
            return J(e);
        }
        continue_parse() {
            try {
                const r = S.__wbindgen_add_to_stack_pointer(-16);
                S.document_continue_parse(r, this.__wbg_ptr);
                var e = j().getInt32(r + 4 * 0, !0), n = j().getInt32(r + 4 * 1, !0);
                if (n) throw J(e);
            } finally{
                S.__wbindgen_add_to_stack_pointer(16);
            }
        }
        get frontmatter() {
            const e = S.document_frontmatter(this.__wbg_ptr);
            return J(e);
        }
        query_headings() {
            const e = S.document_query_headings(this.__wbg_ptr);
            return J(e);
        }
        query_links() {
            const e = S.document_query_links(this.__wbg_ptr);
            return J(e);
        }
        get tags() {
            const e = S.document_tags(this.__wbg_ptr);
            return J(e);
        }
        to_html() {
            let e, n;
            try {
                const o = S.__wbindgen_add_to_stack_pointer(-16);
                S.document_to_html(o, this.__wbg_ptr);
                var r = j().getInt32(o + 4 * 0, !0), i = j().getInt32(o + 4 * 1, !0);
                return e = r, n = i, Ce(r, i);
            } finally{
                S.__wbindgen_add_to_stack_pointer(16), S.__wbindgen_export4(e, n, 1);
            }
        }
        get total_nodes() {
            return S.document_total_nodes(this.__wbg_ptr) >>> 0;
        }
    };
    Symbol.dispose && (Le.prototype[Symbol.dispose] = Le.prototype.free);
    function Ai(t) {
        const e = fe(t, S.__wbindgen_export, S.__wbindgen_export2), n = Z, r = S.parse(e, n);
        return Le.__wrap(r);
    }
    function Ti(t, e) {
        try {
            const o = S.__wbindgen_add_to_stack_pointer(-16), s = fe(t, S.__wbindgen_export, S.__wbindgen_export2), a = Z, l = Oo(e, S.__wbindgen_export), c = Z;
            S.parse_selected(o, s, a, l, c);
            var n = j().getInt32(o + 4 * 0, !0), r = j().getInt32(o + 4 * 1, !0), i = j().getInt32(o + 4 * 2, !0);
            if (i) throw J(r);
            return Le.__wrap(n);
        } finally{
            S.__wbindgen_add_to_stack_pointer(16);
        }
    }
    function Ri(t, e) {
        const n = fe(t, S.__wbindgen_export, S.__wbindgen_export2), r = Z, i = S.parse_with_options(n, r, k(e));
        return Le.__wrap(i);
    }
    function Li(t) {
        const e = fe(t, S.__wbindgen_export, S.__wbindgen_export2), n = Z, r = S.query_semantic_targets(e, n);
        return J(r);
    }
    function Pi(t, e) {
        const n = fe(t, S.__wbindgen_export, S.__wbindgen_export2), r = Z, i = S.query_semantic_targets_with_options(n, r, k(e));
        return J(i);
    }
    function ki() {
        let t, e;
        try {
            const i = S.__wbindgen_add_to_stack_pointer(-16);
            S.version(i);
            var n = j().getInt32(i + 4 * 0, !0), r = j().getInt32(i + 4 * 1, !0);
            return t = n, e = r, Ce(n, r);
        } finally{
            S.__wbindgen_add_to_stack_pointer(16), S.__wbindgen_export4(t, e, 1);
        }
    }
    function Ci(t, e) {
        const n = Error(Ce(t, e));
        return k(n);
    }
    function Ii(t) {
        return Number(A(t));
    }
    function Oi(t, e) {
        const n = A(e), r = typeof n == "bigint" ? n : void 0;
        j().setBigInt64(t + 8 * 1, Pe(r) ? BigInt(0) : r, !0), j().setInt32(t + 4 * 0, !Pe(r), !0);
    }
    function xi(t) {
        const e = A(t), n = typeof e == "boolean" ? e : void 0;
        return Pe(n) ? 16777215 : n ? 1 : 0;
    }
    function Di(t, e) {
        const n = zt(A(e)), r = fe(n, S.__wbindgen_export, S.__wbindgen_export2), i = Z;
        j().setInt32(t + 4 * 1, i, !0), j().setInt32(t + 4 * 0, r, !0);
    }
    function Ni(t, e) {
        return A(t) in A(e);
    }
    function Vi(t) {
        return typeof A(t) == "bigint";
    }
    function Mi(t) {
        return typeof A(t) == "function";
    }
    function $i(t) {
        const e = A(t);
        return typeof e == "object" && e !== null;
    }
    function ji(t) {
        return typeof A(t) == "string";
    }
    function Bi(t) {
        return A(t) === void 0;
    }
    function Gi(t, e) {
        return A(t) === A(e);
    }
    function Ui(t, e) {
        return A(t) == A(e);
    }
    function Hi(t, e) {
        const n = A(e), r = typeof n == "number" ? n : void 0;
        j().setFloat64(t + 8 * 1, Pe(r) ? 0 : r, !0), j().setInt32(t + 4 * 0, !Pe(r), !0);
    }
    function Fi(t, e) {
        const n = A(e), r = typeof n == "string" ? n : void 0;
        var i = Pe(r) ? 0 : fe(r, S.__wbindgen_export, S.__wbindgen_export2), o = Z;
        j().setInt32(t + 4 * 1, o, !0), j().setInt32(t + 4 * 0, i, !0);
    }
    function Wi(t, e) {
        throw new Error(Ce(t, e));
    }
    function qi() {
        return Lt(function(t, e) {
            const n = A(t).call(A(e));
            return k(n);
        }, arguments);
    }
    function zi(t) {
        return A(t).done;
    }
    function Ki(t) {
        const e = Object.entries(A(t));
        return k(e);
    }
    function Ji(t, e) {
        let n, r;
        try {
            n = t, r = e, console.error(Ce(t, e));
        } finally{
            S.__wbindgen_export4(n, r, 1);
        }
    }
    function Xi(t, e) {
        const n = A(t)[e >>> 0];
        return k(n);
    }
    function Yi() {
        return Lt(function(t, e) {
            const n = Reflect.get(A(t), A(e));
            return k(n);
        }, arguments);
    }
    function Qi(t, e) {
        const n = A(t)[e >>> 0];
        return k(n);
    }
    function Zi(t, e) {
        const n = A(t)[A(e)];
        return k(n);
    }
    function eo(t) {
        let e;
        try {
            e = A(t) instanceof ArrayBuffer;
        } catch  {
            e = !1;
        }
        return e;
    }
    function to(t) {
        let e;
        try {
            e = A(t) instanceof Uint8Array;
        } catch  {
            e = !1;
        }
        return e;
    }
    function no(t) {
        return Array.isArray(A(t));
    }
    function ro(t) {
        return Number.isSafeInteger(A(t));
    }
    function io() {
        return k(Symbol.iterator);
    }
    function oo(t) {
        return A(t).length;
    }
    function so(t) {
        return A(t).length;
    }
    function ao() {
        const t = new Error;
        return k(t);
    }
    function lo() {
        const t = new Array;
        return k(t);
    }
    function co() {
        return k(new Map);
    }
    function uo(t) {
        const e = new Uint8Array(A(t));
        return k(e);
    }
    function _o() {
        const t = new Object;
        return k(t);
    }
    function ho(t) {
        const e = A(t).next;
        return k(e);
    }
    function fo() {
        return Lt(function(t) {
            const e = A(t).next();
            return k(e);
        }, arguments);
    }
    function po(t, e, n) {
        Uint8Array.prototype.set.call(dr(t, e), A(n));
    }
    function mo(t, e, n) {
        const r = A(t).set(A(e), A(n));
        return k(r);
    }
    function go(t, e, n) {
        A(t)[J(e)] = J(n);
    }
    function bo() {
        return Lt(function(t, e, n) {
            return Reflect.set(A(t), A(e), A(n));
        }, arguments);
    }
    function yo(t, e, n) {
        A(t)[e >>> 0] = J(n);
    }
    function wo(t, e) {
        const n = A(e).stack, r = fe(n, S.__wbindgen_export, S.__wbindgen_export2), i = Z;
        j().setInt32(t + 4 * 1, i, !0), j().setInt32(t + 4 * 0, r, !0);
    }
    function Eo(t) {
        const e = A(t).value;
        return k(e);
    }
    function vo(t) {
        return k(t);
    }
    function So(t) {
        return k(t);
    }
    function Ao(t, e) {
        const n = Io(t, e);
        return k(n);
    }
    function To(t, e) {
        const n = dr(t, e);
        return k(n);
    }
    function Ro(t, e) {
        const n = Ce(t, e);
        return k(n);
    }
    function Lo(t) {
        const e = BigInt.asUintN(64, t);
        return k(e);
    }
    function Po(t) {
        const e = A(t);
        return k(e);
    }
    function ko(t) {
        J(t);
    }
    const Ln = typeof FinalizationRegistry > "u" ? {
        register: ()=>{},
        unregister: ()=>{}
    } : new FinalizationRegistry((t)=>S.__wbg_document_free(t, 1));
    function k(t) {
        je === se.length && se.push(se.length + 1);
        const e = je;
        return je = se[e], se[e] = t, e;
    }
    function zt(t) {
        const e = typeof t;
        if (e == "number" || e == "boolean" || t == null) return `${t}`;
        if (e == "string") return `"${t}"`;
        if (e == "symbol") {
            const i = t.description;
            return i == null ? "Symbol" : `Symbol(${i})`;
        }
        if (e == "function") {
            const i = t.name;
            return typeof i == "string" && i.length > 0 ? `Function(${i})` : "Function";
        }
        if (Array.isArray(t)) {
            const i = t.length;
            let o = "[";
            i > 0 && (o += zt(t[0]));
            for(let s = 1; s < i; s++)o += ", " + zt(t[s]);
            return o += "]", o;
        }
        const n = /\[object ([^\]]+)\]/.exec(toString.call(t));
        let r;
        if (n && n.length > 1) r = n[1];
        else return toString.call(t);
        if (r == "Object") try {
            return "Object(" + JSON.stringify(t) + ")";
        } catch  {
            return "Object";
        }
        return t instanceof Error ? `${t.name}: ${t.message}
${t.stack}` : r;
    }
    function Co(t) {
        t < 1028 || (se[t] = je, je = t);
    }
    function Io(t, e) {
        return t = t >>> 0, _r().subarray(t / 4, t / 4 + e);
    }
    function dr(t, e) {
        return t = t >>> 0, $e().subarray(t / 1, t / 1 + e);
    }
    let Se = null;
    function j() {
        return (Se === null || Se.buffer.detached === !0 || Se.buffer.detached === void 0 && Se.buffer !== S.memory.buffer) && (Se = new DataView(S.memory.buffer)), Se;
    }
    function Ce(t, e) {
        return Do(t >>> 0, e);
    }
    let et = null;
    function _r() {
        return (et === null || et.byteLength === 0) && (et = new Uint32Array(S.memory.buffer)), et;
    }
    let tt = null;
    function $e() {
        return (tt === null || tt.byteLength === 0) && (tt = new Uint8Array(S.memory.buffer)), tt;
    }
    function A(t) {
        return se[t];
    }
    function Lt(t, e) {
        try {
            return t.apply(this, e);
        } catch (n) {
            S.__wbindgen_export3(k(n));
        }
    }
    let se = new Array(1024).fill(void 0);
    se.push(void 0, null, !0, !1);
    let je = se.length;
    function Pe(t) {
        return t == null;
    }
    function Oo(t, e) {
        const n = e(t.length * 4, 4) >>> 0;
        return _r().set(t, n / 4), Z = t.length, n;
    }
    function fe(t, e, n) {
        if (n === void 0) {
            const a = Be.encode(t), l = e(a.length, 1) >>> 0;
            return $e().subarray(l, l + a.length).set(a), Z = a.length, l;
        }
        let r = t.length, i = e(r, 1) >>> 0;
        const o = $e();
        let s = 0;
        for(; s < r; s++){
            const a = t.charCodeAt(s);
            if (a > 127) break;
            o[i + s] = a;
        }
        if (s !== r) {
            s !== 0 && (t = t.slice(s)), i = n(i, r, r = s + t.length * 3, 1) >>> 0;
            const a = $e().subarray(i + s, i + r), l = Be.encodeInto(t, a);
            s += l.written, i = n(i, r, s, 1) >>> 0;
        }
        return Z = s, i;
    }
    function J(t) {
        const e = A(t);
        return Co(t), e;
    }
    let lt = new TextDecoder("utf-8", {
        ignoreBOM: !0,
        fatal: !0
    });
    lt.decode();
    const xo = 2146435072;
    let Vt = 0;
    function Do(t, e) {
        return Vt += e, Vt >= xo && (lt = new TextDecoder("utf-8", {
            ignoreBOM: !0,
            fatal: !0
        }), lt.decode(), Vt = e), lt.decode($e().subarray(t, t + e));
    }
    const Be = new TextEncoder;
    "encodeInto" in Be || (Be.encodeInto = function(t, e) {
        const n = Be.encode(t);
        return e.set(n), {
            read: t.length,
            written: n.length
        };
    });
    let Z = 0, S;
    function No(t) {
        S = t;
    }
    URL = globalThis.URL;
    const Vo = await Si({
        "./markdown_binding_bg.js": {
            __wbg___wbindgen_string_get_b0ca35b86a603356: Fi,
            __wbindgen_object_drop_ref: ko,
            __wbg___wbindgen_jsval_loose_eq_db4c3b15f63fc170: Ui,
            __wbg_new_da52cf8fe3429cb2: _o,
            __wbg_new_32b398fb48b6d94a: lo,
            __wbg_set_8a16b38e4805b298: yo,
            __wbg_set_8535240470bf2500: bo,
            __wbg_new_7796ffc7ed656783: co,
            __wbg_set_575dd786d51585f8: mo,
            __wbg_set_6be42768c690e380: go,
            __wbg_new_227d7c05414eb861: ao,
            __wbg_stack_3b0d974bbf31e44f: wo,
            __wbg_error_a6fa202b58aa1cd3: Ji,
            __wbg_length_1f0964f4a5e2c6d8: oo,
            __wbg_prototypesetcall_4770620bbe4688a0: po,
            __wbg_Error_92b29b0548f8b746: Ci,
            __wbg___wbindgen_is_bigint_2f76dc55065b4273: Vi,
            __wbg_isSafeInteger_04f36e4056f1b851: ro,
            __wbg_Number_9a4e0ecb0fa16705: Ii,
            __wbg___wbindgen_bigint_get_as_i64_d968e41184ae354f: Oi,
            __wbg___wbindgen_jsval_eq_e659fcf7b0e32763: Gi,
            __wbg___wbindgen_is_object_a27215656b807791: $i,
            __wbg_get_with_ref_key_6412cf3094599694: Zi,
            __wbg___wbindgen_is_undefined_c05833b95a3cf397: Bi,
            __wbg___wbindgen_in_aca499c5de7ff5e5: Ni,
            __wbg_isArray_0677c962b281d01a: no,
            __wbg_length_370319915dc99107: so,
            __wbg_get_unchecked_6e0ad6d2a41b06f6: Qi,
            __wbg_iterator_6f722e4a93058b71: io,
            __wbg_get_c7eb1f358a7654df: Yi,
            __wbg___wbindgen_is_function_1ff95bcc5517c252: Mi,
            __wbg_call_8a2dd23819f8a60a: qi,
            __wbg_next_6dbf2c0ac8cde20f: ho,
            __wbg_next_71f2aa1cb3d1e37e: fo,
            __wbg_done_89b2b13e91a60321: zi,
            __wbg_value_a5d5488a9589444a: Eo,
            __wbg___wbindgen_boolean_get_fa956cfa2d1bd751: xi,
            __wbg___wbindgen_is_string_ea5e6cc2e4141dfe: ji,
            __wbg_entries_015dc610cd81ede0: Ki,
            __wbg_get_507a50627bffa49b: Xi,
            __wbindgen_object_clone_ref: Po,
            __wbg___wbindgen_number_get_394265ed1e1b84ee: Hi,
            __wbg_instanceof_Uint8Array_309b927aaf7a3fc7: to,
            __wbg_instanceof_ArrayBuffer_4480b9e0068a8adb: eo,
            __wbg_new_cd45aabdf6073e84: uo,
            __wbg___wbindgen_throw_344f42d3211c4765: Wi,
            __wbg___wbindgen_debug_string_c25d447a39f5578f: Di,
            __wbindgen_cast_0000000000000001: vo,
            __wbindgen_cast_0000000000000002: So,
            __wbindgen_cast_0000000000000003: Ao,
            __wbindgen_cast_0000000000000004: To,
            __wbindgen_cast_0000000000000005: Ro,
            __wbindgen_cast_0000000000000006: Lo
        }
    }, vi), { memory: Mo, __wbg_document_free: $o, document_astData: jo, document_continue_parse: Bo, document_frontmatter: Go, document_query_headings: Uo, document_query_links: Ho, document_tags: Fo, document_to_html: Wo, document_total_nodes: qo, parse: zo, parse_selected: Ko, parse_with_options: Jo, query_semantic_targets: Xo, query_semantic_targets_with_options: Yo, version: Qo, __abort_handler: Zo, __instance_terminated: es, __wbindgen_export: ts, __wbindgen_export2: ns, __wbindgen_export3: rs, __wbindgen_export4: is, __wbindgen_add_to_stack_pointer: os } = Vo, ss = Object.freeze(Object.defineProperty({
        __proto__: null,
        __abort_handler: Zo,
        __instance_terminated: es,
        __wbg_document_free: $o,
        __wbindgen_add_to_stack_pointer: os,
        __wbindgen_export: ts,
        __wbindgen_export2: ns,
        __wbindgen_export3: rs,
        __wbindgen_export4: is,
        document_astData: jo,
        document_continue_parse: Bo,
        document_frontmatter: Go,
        document_query_headings: Uo,
        document_query_links: Ho,
        document_tags: Fo,
        document_to_html: Wo,
        document_total_nodes: qo,
        memory: Mo,
        parse: zo,
        parse_selected: Ko,
        parse_with_options: Jo,
        query_semantic_targets: Xo,
        query_semantic_targets_with_options: Yo,
        version: Qo
    }, Symbol.toStringTag, {
        value: "Module"
    }));
    No(ss);
    const as = Object.freeze(Object.defineProperty({
        __proto__: null,
        Document: Le,
        parse: Ai,
        parse_selected: Ti,
        parse_with_options: Ri,
        query_semantic_targets: Li,
        query_semantic_targets_with_options: Pi,
        version: ki
    }, Symbol.toStringTag, {
        value: "Module"
    })), ls = 4294967295;
    function cs(t) {
        return t <= 127 ? 1 : t <= 2047 ? 2 : t <= 65535 ? 3 : 4;
    }
    function us(t, e, n) {
        const r = [];
        for(let c = 0; c < e.length; c += 1)r.push([
            e[c],
            c,
            !0
        ], [
            n[c],
            c,
            !1
        ]);
        r.sort((c, d)=>c[0] - d[0]);
        const i = new Array(r.length);
        let o = 0, s = 0, a = 1, l = 1;
        for (const [c, d, _] of r){
            for(; o < c && s < t.length;){
                const p = t.codePointAt(s), f = cs(p);
                if (o + f > c) break;
                o += f, s += p > 65535 ? 2 : 1, p === 10 ? (a += 1, l = 1) : l += 1;
            }
            i[d * 2 + (_ ? 0 : 1)] = {
                line: a,
                column: l
            };
        }
        return i;
    }
    function ds(t, e) {
        if (t.abi_version !== 1) throw new Error(`unsupported AST transport ABI: ${t.abi_version}`);
        const n = JSON.parse(t.payloads_json);
        if (n.length !== t.node_count) throw new Error("invalid AST transport: payload count does not match node count");
        const r = us(e, t.start, t.end), i = new Array(t.node_count);
        for(let o = 0; o < t.node_count; o += 1){
            const s = n[o], a = {
                kind: t.kind_names[t.kind[o]],
                start: r[o * 2],
                end: r[o * 2 + 1],
                children: []
            };
            s.id !== null && (a.id = s.id), Object.hasOwn(s, "content") && (a.content = s.content), i[o] = a;
        }
        for(let o = 0; o < t.node_count; o += 1){
            let s = t.first_child[o];
            for(; s !== ls;)i[o].children.push(i[s]), s = t.next_sibling[s];
        }
        return i[t.root];
    }
    class nt {
        #e;
        #i;
        #n;
        #r = !1;
        constructor(e, n){
            this.#e = e, this.#i = n;
        }
        #t() {
            if (this.#r) throw new Error("document has been disposed");
        }
        get tree() {
            return this.#t(), this.#n === void 0 && (this.#n = ds(this.#e.astData(), this.#i)), this.#n;
        }
        get tags() {
            return this.#t(), this.#e.tags;
        }
        get frontmatter() {
            return this.#t(), this.#e.frontmatter;
        }
        get totalNodes() {
            return this.#t(), this.#e.total_nodes;
        }
        toHtml() {
            return this.#t(), this.#e.to_html();
        }
        queryHeadings() {
            return this.#t(), this.#e.query_headings();
        }
        queryLinks() {
            return this.#t(), this.#e.query_links();
        }
        continueParse() {
            this.#t(), this.#e.continue_parse(), this.#n = void 0;
        }
        dispose() {
            this.#r || (this.#e.free(), this.#r = !0, this.#n = void 0);
        }
    }
    function _s(t) {
        return {
            Document: nt,
            parse (e) {
                return new nt(t.parse(e), e);
            },
            parseWithOptions (e, n) {
                return new nt(t.parse_with_options(e, n), e);
            },
            parseSelected (e, n) {
                return new nt(t.parse_selected(e, n), e);
            },
            querySemanticTargets (e) {
                return t.query_semantic_targets(e);
            },
            querySemanticTargetsWithOptions (e, n) {
                return t.query_semantic_targets_with_options(e, n);
            },
            version: t.version
        };
    }
    const hs = _s(as), fs = hs.parseWithOptions;
    var ps = N("<span class=json-array-label>"), ms = N("<span class=json-colon> "), gs = N("<span class=json-preview> <!> "), bs = N("<span class=json-bracket>"), Mt = N("<span class=json-comma>,"), ys = N('<div class=json-line><button class=json-toggle><svg width=12 height=12 viewBox="0 0 12 12"><path d="M4 2 L8 6 L4 10"fill=none stroke=currentColor stroke-width=1.5></path></svg></button><span class=json-bracket>'), ws = N("<div class=json-children>"), Es = N('<div class="json-line json-closing-bracket"><span class=json-spacer></span><span class=json-bracket>'), Pn = N('<span class=json-key>"<!>"'), kn = N("<span class=json-colon>: "), vs = N("<div class=json-line><span class=json-spacer></span><span>");
    function Kt(t) {
        const [e, n] = te(t.depth ? t.depth > 1 : !1), r = t.depth || 0, i = (h)=>h !== null && typeof h == "object" && !Array.isArray(h), o = (h)=>Array.isArray(h), s = (h)=>!i(h) && !o(h), a = (h)=>h === null ? "json-null" : h === void 0 ? "json-undefined" : typeof h == "string" ? "json-string" : typeof h == "number" ? "json-number" : typeof h == "boolean" ? "json-boolean" : "", l = (h)=>h === null ? "null" : h === void 0 ? "undefined" : typeof h == "string" ? `"${h}"` : String(h), c = (h)=>{
            if (o(h)) return h.length === 0 ? "" : `${h.length} items`;
            if (i(h)) {
                const E = Object.keys(h);
                return E.length === 0 ? "" : `${E.length} keys`;
            }
            return "";
        }, d = ()=>{
            if (t.arrayIndex !== void 0 && i(t.data) && t.data.kind) return t.data.kind;
        }, _ = Ae(()=>i(t.data) ? Object.entries(t.data) : o(t.data) ? t.data.map((h, E)=>[
                    E,
                    h
                ]) : []), p = (h)=>{
            h.stopPropagation(), t.onNodeClick && i(t.data) && t.onNodeClick(t.data.start, t.data.end);
        }, f = t.name || d();
        return [
            I(U, {
                get when () {
                    return i(t.data) || o(t.data);
                },
                get children () {
                    return [
                        (()=>{
                            var h = ys(), E = h.firstChild, g = E.firstChild, w = E.nextSibling;
                            return h.$$click = p, E.$$click = (m)=>{
                                m.stopPropagation(), n(!e());
                            }, L(h, I(U, {
                                when: f !== void 0,
                                get children () {
                                    return I(U, {
                                        get when () {
                                            return t.arrayIndex !== void 0;
                                        },
                                        get fallback () {
                                            return [
                                                (()=>{
                                                    var m = Pn(), y = m.firstChild, v = y.nextSibling;
                                                    return v.nextSibling, L(m, f, v), m;
                                                })(),
                                                kn()
                                            ];
                                        },
                                        get children () {
                                            return [
                                                (()=>{
                                                    var m = ps();
                                                    return L(m, f), m;
                                                })(),
                                                ms()
                                            ];
                                        }
                                    });
                                }
                            }), w), L(w, ()=>o(t.data) ? "[" : "{"), L(h, I(U, {
                                get when () {
                                    return e();
                                },
                                get children () {
                                    return [
                                        (()=>{
                                            var m = gs(), y = m.firstChild, v = y.nextSibling;
                                            return v.nextSibling, L(m, ()=>c(t.data), v), m;
                                        })(),
                                        (()=>{
                                            var m = bs();
                                            return L(m, ()=>o(t.data) ? "]" : "}"), m;
                                        })()
                                    ];
                                }
                            }), null), L(h, I(U, {
                                get when () {
                                    return Me(()=>!t.isLast)() && e();
                                },
                                get children () {
                                    return Mt();
                                }
                            }), null), ie((m)=>{
                                var y = e() ? "Expand" : "Collapse", v = `json-arrow ${e() ? "json-arrow-collapsed" : "json-arrow-expanded"}`;
                                return y !== m.e && Tn(E, "title", m.e = y), v !== m.t && Tn(g, "class", m.t = v), m;
                            }, {
                                e: void 0,
                                t: void 0
                            }), h;
                        })(),
                        I(U, {
                            get when () {
                                return !e();
                            },
                            get children () {
                                return [
                                    (()=>{
                                        var h = ws();
                                        return L(h, I(lr, {
                                            get each () {
                                                return _();
                                            },
                                            children: ([E, g], w)=>I(Kt, {
                                                    get name () {
                                                        return Me(()=>!!o(t.data))() ? void 0 : String(E);
                                                    },
                                                    data: g,
                                                    depth: r + 1,
                                                    get isLast () {
                                                        return w() === _().length - 1;
                                                    },
                                                    get onNodeClick () {
                                                        return t.onNodeClick;
                                                    },
                                                    get arrayIndex () {
                                                        return Me(()=>!!o(t.data))() ? Number(E) : void 0;
                                                    }
                                                })
                                        })), h;
                                    })(),
                                    (()=>{
                                        var h = Es(), E = h.firstChild, g = E.nextSibling;
                                        return L(g, ()=>o(t.data) ? "]" : "}"), L(h, I(U, {
                                            get when () {
                                                return !t.isLast;
                                            },
                                            get children () {
                                                return Mt();
                                            }
                                        }), null), h;
                                    })()
                                ];
                            }
                        })
                    ];
                }
            }),
            I(U, {
                get when () {
                    return s(t.data);
                },
                get children () {
                    var h = vs(), E = h.firstChild, g = E.nextSibling;
                    return L(h, I(U, {
                        get when () {
                            return t.name !== void 0;
                        },
                        get children () {
                            return [
                                (()=>{
                                    var w = Pn(), m = w.firstChild, y = m.nextSibling;
                                    return y.nextSibling, L(w, ()=>t.name, y), w;
                                })(),
                                kn()
                            ];
                        }
                    }), g), L(g, ()=>l(t.data)), L(h, I(U, {
                        get when () {
                            return !t.isLast;
                        },
                        get children () {
                            return Mt();
                        }
                    }), null), ie(()=>Ne(g, a(t.data))), h;
                }
            })
        ];
    }
    cr([
        "click"
    ]);
    const Ss = "modulepreload", As = function(t) {
        return "/markdown/" + t;
    }, Cn = {}, u = function(e, n, r) {
        let i = Promise.resolve();
        if (n && n.length > 0) {
            let s = function(c) {
                return Promise.all(c.map((d)=>Promise.resolve(d).then((_)=>({
                            status: "fulfilled",
                            value: _
                        }), (_)=>({
                            status: "rejected",
                            reason: _
                        }))));
            };
            document.getElementsByTagName("link");
            const a = document.querySelector("meta[property=csp-nonce]"), l = a?.nonce || a?.getAttribute("nonce");
            i = s(n.map((c)=>{
                if (c = As(c), c in Cn) return;
                Cn[c] = !0;
                const d = c.endsWith(".css"), _ = d ? '[rel="stylesheet"]' : "";
                if (document.querySelector(`link[href="${c}"]${_}`)) return;
                const p = document.createElement("link");
                if (p.rel = d ? "stylesheet" : Ss, d || (p.as = "script"), p.crossOrigin = "", p.href = c, l && p.setAttribute("nonce", l), document.head.appendChild(p), d) return new Promise((f, h)=>{
                    p.addEventListener("load", f), p.addEventListener("error", ()=>h(new Error(`Unable to preload CSS for ${c}`)));
                });
            }));
        }
        function o(s) {
            const a = new Event("vite:preloadError", {
                cancelable: !0
            });
            if (a.payload = s, window.dispatchEvent(a), !a.defaultPrevented) throw s;
        }
        return i.then((s)=>{
            for (const a of s || [])a.status === "rejected" && o(a.reason);
            return e().catch(o);
        });
    }, hr = [
        {
            id: "abap",
            name: "ABAP",
            import: ()=>u(()=>import("./abap-DsBKuouk.js"), [])
        },
        {
            id: "actionscript-3",
            name: "ActionScript",
            import: ()=>u(()=>import("./actionscript-3-D_z4Izcz.js"), [])
        },
        {
            id: "ada",
            name: "Ada",
            import: ()=>u(()=>import("./ada-727ZlQH0.js"), [])
        },
        {
            id: "angular-html",
            name: "Angular HTML",
            import: ()=>u(()=>import("./angular-html-LfdN0zeE.js").then((t)=>t.f), __vite__mapDeps([0,1,2,3]))
        },
        {
            id: "angular-ts",
            name: "Angular TypeScript",
            import: ()=>u(()=>import("./angular-ts-CKsD7JZE.js"), __vite__mapDeps([4,0,1,2,3,5]))
        },
        {
            id: "apache",
            name: "Apache Conf",
            import: ()=>u(()=>import("./apache-Dn00JSTd.js"), [])
        },
        {
            id: "apex",
            name: "Apex",
            import: ()=>u(()=>import("./apex-COJ4H7py.js"), [])
        },
        {
            id: "apl",
            name: "APL",
            import: ()=>u(()=>import("./apl-BBq3IX1j.js"), __vite__mapDeps([6,1,2,3,7,8,9]))
        },
        {
            id: "applescript",
            name: "AppleScript",
            import: ()=>u(()=>import("./applescript-Bu5BbsvL.js"), [])
        },
        {
            id: "ara",
            name: "Ara",
            import: ()=>u(()=>import("./ara-7O62HKoU.js"), [])
        },
        {
            id: "asciidoc",
            name: "AsciiDoc",
            aliases: [
                "adoc"
            ],
            import: ()=>u(()=>import("./asciidoc-BPT9niGB.js"), [])
        },
        {
            id: "asm",
            name: "Assembly",
            import: ()=>u(()=>import("./asm-Dhn9LcZ4.js"), [])
        },
        {
            id: "astro",
            name: "Astro",
            import: ()=>u(()=>import("./astro-CqkE3fuf.js"), __vite__mapDeps([10,9,2,11,3,12]))
        },
        {
            id: "awk",
            name: "AWK",
            import: ()=>u(()=>import("./awk-eg146-Ew.js"), [])
        },
        {
            id: "ballerina",
            name: "Ballerina",
            import: ()=>u(()=>import("./ballerina-Du268qiB.js"), [])
        },
        {
            id: "bat",
            name: "Batch File",
            aliases: [
                "batch"
            ],
            import: ()=>u(()=>import("./bat-fje9CFhw.js"), [])
        },
        {
            id: "beancount",
            name: "Beancount",
            import: ()=>u(()=>import("./beancount-BwXTMy5W.js"), [])
        },
        {
            id: "berry",
            name: "Berry",
            aliases: [
                "be"
            ],
            import: ()=>u(()=>import("./berry-3xVqZejG.js"), [])
        },
        {
            id: "bibtex",
            name: "BibTeX",
            import: ()=>u(()=>import("./bibtex-xW4inM5L.js"), [])
        },
        {
            id: "bicep",
            name: "Bicep",
            import: ()=>u(()=>import("./bicep-DHo0CJ0O.js"), [])
        },
        {
            id: "blade",
            name: "Blade",
            import: ()=>u(()=>import("./blade-a8OxSdnT.js"), __vite__mapDeps([13,1,2,3,7,8,14,9]))
        },
        {
            id: "bsl",
            name: "1C (Enterprise)",
            aliases: [
                "1c"
            ],
            import: ()=>u(()=>import("./bsl-Dgyn0ogV.js"), __vite__mapDeps([15,16]))
        },
        {
            id: "c",
            name: "C",
            import: ()=>u(()=>import("./c-C3t2pwGQ.js"), [])
        },
        {
            id: "cadence",
            name: "Cadence",
            aliases: [
                "cdc"
            ],
            import: ()=>u(()=>import("./cadence-DNquZEk8.js"), [])
        },
        {
            id: "cairo",
            name: "Cairo",
            import: ()=>u(()=>import("./cairo--RitsXJZ.js"), __vite__mapDeps([17,18]))
        },
        {
            id: "clarity",
            name: "Clarity",
            import: ()=>u(()=>import("./clarity-BHOwM8T6.js"), [])
        },
        {
            id: "clojure",
            name: "Clojure",
            aliases: [
                "clj"
            ],
            import: ()=>u(()=>import("./clojure-DxSadP1t.js"), [])
        },
        {
            id: "cmake",
            name: "CMake",
            import: ()=>u(()=>import("./cmake-DbXoA79R.js"), [])
        },
        {
            id: "cobol",
            name: "COBOL",
            import: ()=>u(()=>import("./cobol-PTqiYgYu.js"), __vite__mapDeps([19,1,2,3,8]))
        },
        {
            id: "codeowners",
            name: "CODEOWNERS",
            import: ()=>u(()=>import("./codeowners-Bp6g37R7.js"), [])
        },
        {
            id: "codeql",
            name: "CodeQL",
            aliases: [
                "ql"
            ],
            import: ()=>u(()=>import("./codeql-sacFqUAJ.js"), [])
        },
        {
            id: "coffee",
            name: "CoffeeScript",
            aliases: [
                "coffeescript"
            ],
            import: ()=>u(()=>import("./coffee-dyiR41kL.js"), __vite__mapDeps([20,2]))
        },
        {
            id: "common-lisp",
            name: "Common Lisp",
            aliases: [
                "lisp"
            ],
            import: ()=>u(()=>import("./common-lisp-C7gG9l05.js"), [])
        },
        {
            id: "coq",
            name: "Coq",
            import: ()=>u(()=>import("./coq-Dsg_Bt_b.js"), [])
        },
        {
            id: "cpp",
            name: "C++",
            aliases: [
                "c++"
            ],
            import: ()=>u(()=>import("./cpp-BksuvNSY.js"), __vite__mapDeps([21,22,23,24,14]))
        },
        {
            id: "crystal",
            name: "Crystal",
            import: ()=>u(()=>import("./crystal-DtDmRg-F.js"), __vite__mapDeps([25,1,2,3,14,24,26]))
        },
        {
            id: "csharp",
            name: "C#",
            aliases: [
                "c#",
                "cs"
            ],
            import: ()=>u(()=>import("./csharp-D9R-vmeu.js"), [])
        },
        {
            id: "css",
            name: "CSS",
            import: ()=>u(()=>import("./css-BPhBrDlE.js"), [])
        },
        {
            id: "csv",
            name: "CSV",
            import: ()=>u(()=>import("./csv-B0qRVHPH.js"), [])
        },
        {
            id: "cue",
            name: "CUE",
            import: ()=>u(()=>import("./cue-DtFQj3wx.js"), [])
        },
        {
            id: "cypher",
            name: "Cypher",
            aliases: [
                "cql"
            ],
            import: ()=>u(()=>import("./cypher-m2LEI-9-.js"), [])
        },
        {
            id: "d",
            name: "D",
            import: ()=>u(()=>import("./d-BoXegm-a.js"), [])
        },
        {
            id: "dart",
            name: "Dart",
            import: ()=>u(()=>import("./dart-B9wLZaAG.js"), [])
        },
        {
            id: "dax",
            name: "DAX",
            import: ()=>u(()=>import("./dax-ClGRhx96.js"), [])
        },
        {
            id: "desktop",
            name: "Desktop",
            import: ()=>u(()=>import("./desktop-DEIpsLCJ.js"), [])
        },
        {
            id: "diff",
            name: "Diff",
            import: ()=>u(()=>import("./diff-BgYniUM_.js"), [])
        },
        {
            id: "docker",
            name: "Dockerfile",
            aliases: [
                "dockerfile"
            ],
            import: ()=>u(()=>import("./docker-COcR7UxN.js"), [])
        },
        {
            id: "dotenv",
            name: "dotEnv",
            import: ()=>u(()=>import("./dotenv-BjQB5zDj.js"), [])
        },
        {
            id: "dream-maker",
            name: "Dream Maker",
            import: ()=>u(()=>import("./dream-maker-C-nORZOA.js"), [])
        },
        {
            id: "edge",
            name: "Edge",
            import: ()=>u(()=>import("./edge-D5gP-w-T.js"), __vite__mapDeps([27,11,1,2,3,28]))
        },
        {
            id: "elixir",
            name: "Elixir",
            import: ()=>u(()=>import("./elixir-CLiX3zqd.js"), __vite__mapDeps([29,1,2,3]))
        },
        {
            id: "elm",
            name: "Elm",
            import: ()=>u(()=>import("./elm-CmHSxxaM.js"), __vite__mapDeps([30,23,24]))
        },
        {
            id: "emacs-lisp",
            name: "Emacs Lisp",
            aliases: [
                "elisp"
            ],
            import: ()=>u(()=>import("./emacs-lisp-BX77sIaO.js"), [])
        },
        {
            id: "erb",
            name: "ERB",
            import: ()=>u(()=>import("./erb-BYTLMnw6.js"), __vite__mapDeps([31,1,2,3,32,33,7,8,14,34,11,35,36,21,22,23,24,26,37,38]))
        },
        {
            id: "erlang",
            name: "Erlang",
            aliases: [
                "erl"
            ],
            import: ()=>u(()=>import("./erlang-B-DoSBHF.js"), [])
        },
        {
            id: "fennel",
            name: "Fennel",
            import: ()=>u(()=>import("./fennel-bCA53EVm.js"), [])
        },
        {
            id: "fish",
            name: "Fish",
            import: ()=>u(()=>import("./fish-w-ucz2PV.js"), [])
        },
        {
            id: "fluent",
            name: "Fluent",
            aliases: [
                "ftl"
            ],
            import: ()=>u(()=>import("./fluent-Dayu4EKP.js"), [])
        },
        {
            id: "fortran-fixed-form",
            name: "Fortran (Fixed Form)",
            aliases: [
                "f",
                "for",
                "f77"
            ],
            import: ()=>u(()=>import("./fortran-fixed-form-TqA4NnZg.js"), __vite__mapDeps([39,40]))
        },
        {
            id: "fortran-free-form",
            name: "Fortran (Free Form)",
            aliases: [
                "f90",
                "f95",
                "f03",
                "f08",
                "f18"
            ],
            import: ()=>u(()=>import("./fortran-free-form-DKXYxT9g.js"), [])
        },
        {
            id: "fsharp",
            name: "F#",
            aliases: [
                "f#",
                "fs"
            ],
            import: ()=>u(()=>import("./fsharp-XplgxFYe.js"), __vite__mapDeps([41,42]))
        },
        {
            id: "gdresource",
            name: "GDResource",
            import: ()=>u(()=>import("./gdresource-BHYsBjWJ.js"), __vite__mapDeps([43,44,45]))
        },
        {
            id: "gdscript",
            name: "GDScript",
            import: ()=>u(()=>import("./gdscript-DfxzS6Rs.js"), [])
        },
        {
            id: "gdshader",
            name: "GDShader",
            import: ()=>u(()=>import("./gdshader-SKMF96pI.js"), [])
        },
        {
            id: "genie",
            name: "Genie",
            import: ()=>u(()=>import("./genie-ajMbGru0.js"), [])
        },
        {
            id: "gherkin",
            name: "Gherkin",
            import: ()=>u(()=>import("./gherkin--30QC5Em.js"), [])
        },
        {
            id: "git-commit",
            name: "Git Commit Message",
            import: ()=>u(()=>import("./git-commit-i4q6IMui.js"), __vite__mapDeps([46,47]))
        },
        {
            id: "git-rebase",
            name: "Git Rebase Message",
            import: ()=>u(()=>import("./git-rebase-B-v9cOL2.js"), __vite__mapDeps([48,26]))
        },
        {
            id: "gleam",
            name: "Gleam",
            import: ()=>u(()=>import("./gleam-B430Bg39.js"), [])
        },
        {
            id: "glimmer-js",
            name: "Glimmer JS",
            aliases: [
                "gjs"
            ],
            import: ()=>u(()=>import("./glimmer-js-D-cwc0-E.js"), __vite__mapDeps([49,2,11,3,1]))
        },
        {
            id: "glimmer-ts",
            name: "Glimmer TS",
            aliases: [
                "gts"
            ],
            import: ()=>u(()=>import("./glimmer-ts-pgjy16dm.js"), __vite__mapDeps([50,11,3,2,1]))
        },
        {
            id: "glsl",
            name: "GLSL",
            import: ()=>u(()=>import("./glsl-DBO2IWDn.js"), __vite__mapDeps([23,24]))
        },
        {
            id: "gnuplot",
            name: "Gnuplot",
            import: ()=>u(()=>import("./gnuplot-CM8KxXT1.js"), [])
        },
        {
            id: "go",
            name: "Go",
            import: ()=>u(()=>import("./go-B1SYOhNW.js"), [])
        },
        {
            id: "graphql",
            name: "GraphQL",
            aliases: [
                "gql"
            ],
            import: ()=>u(()=>import("./graphql-cDcHW_If.js"), __vite__mapDeps([34,2,11,35,36]))
        },
        {
            id: "groovy",
            name: "Groovy",
            import: ()=>u(()=>import("./groovy-DkBy-JyN.js"), [])
        },
        {
            id: "hack",
            name: "Hack",
            import: ()=>u(()=>import("./hack-D1yCygmZ.js"), __vite__mapDeps([51,1,2,3,14]))
        },
        {
            id: "haml",
            name: "Ruby Haml",
            import: ()=>u(()=>import("./haml-B2EZWmdv.js"), __vite__mapDeps([33,2,3]))
        },
        {
            id: "handlebars",
            name: "Handlebars",
            aliases: [
                "hbs"
            ],
            import: ()=>u(()=>import("./handlebars-BQGss363.js"), __vite__mapDeps([52,1,2,3,38]))
        },
        {
            id: "haskell",
            name: "Haskell",
            aliases: [
                "hs"
            ],
            import: ()=>u(()=>import("./haskell-BILxekzW.js"), [])
        },
        {
            id: "haxe",
            name: "Haxe",
            import: ()=>u(()=>import("./haxe-C5wWYbrZ.js"), [])
        },
        {
            id: "hcl",
            name: "HashiCorp HCL",
            import: ()=>u(()=>import("./hcl-HzYwdGDm.js"), [])
        },
        {
            id: "hjson",
            name: "Hjson",
            import: ()=>u(()=>import("./hjson-T-Tgc4AT.js"), [])
        },
        {
            id: "hlsl",
            name: "HLSL",
            import: ()=>u(()=>import("./hlsl-ifBTmRxC.js"), [])
        },
        {
            id: "html",
            name: "HTML",
            import: ()=>u(()=>import("./html-C2L_23MC.js"), __vite__mapDeps([1,2,3]))
        },
        {
            id: "html-derivative",
            name: "HTML (Derivative)",
            import: ()=>u(()=>import("./html-derivative-CSfWNPLT.js"), __vite__mapDeps([28,1,2,3]))
        },
        {
            id: "http",
            name: "HTTP",
            import: ()=>u(()=>import("./http-FRrOvY1W.js"), __vite__mapDeps([53,26,9,7,8,34,2,11,35,36]))
        },
        {
            id: "hxml",
            name: "HXML",
            import: ()=>u(()=>import("./hxml-TIA70rKU.js"), __vite__mapDeps([54,55]))
        },
        {
            id: "hy",
            name: "Hy",
            import: ()=>u(()=>import("./hy-BMj5Y0dO.js"), [])
        },
        {
            id: "imba",
            name: "Imba",
            import: ()=>u(()=>import("./imba-bv_oIlVt.js"), __vite__mapDeps([56,11]))
        },
        {
            id: "ini",
            name: "INI",
            aliases: [
                "properties"
            ],
            import: ()=>u(()=>import("./ini-BjABl1g7.js"), [])
        },
        {
            id: "java",
            name: "Java",
            import: ()=>u(()=>import("./java-xI-RfyKK.js"), [])
        },
        {
            id: "javascript",
            name: "JavaScript",
            aliases: [
                "js"
            ],
            import: ()=>u(()=>import("./javascript-ySlJ1b_l.js"), [])
        },
        {
            id: "jinja",
            name: "Jinja",
            import: ()=>u(()=>import("./jinja-DGy0s7-h.js"), __vite__mapDeps([57,1,2,3]))
        },
        {
            id: "jison",
            name: "Jison",
            import: ()=>u(()=>import("./jison-BqZprYcd.js"), __vite__mapDeps([58,2]))
        },
        {
            id: "json",
            name: "JSON",
            import: ()=>u(()=>import("./json-BQoSv7ci.js"), [])
        },
        {
            id: "json5",
            name: "JSON5",
            import: ()=>u(()=>import("./json5-w8dY5SsB.js"), [])
        },
        {
            id: "jsonc",
            name: "JSON with Comments",
            import: ()=>u(()=>import("./jsonc-TU54ms6u.js"), [])
        },
        {
            id: "jsonl",
            name: "JSON Lines",
            import: ()=>u(()=>import("./jsonl-DREVFZK8.js"), [])
        },
        {
            id: "jsonnet",
            name: "Jsonnet",
            import: ()=>u(()=>import("./jsonnet-BfivnA6A.js"), [])
        },
        {
            id: "jssm",
            name: "JSSM",
            aliases: [
                "fsl"
            ],
            import: ()=>u(()=>import("./jssm-P4WzXJd0.js"), [])
        },
        {
            id: "jsx",
            name: "JSX",
            import: ()=>u(()=>import("./jsx-BAng5TT0.js"), [])
        },
        {
            id: "julia",
            name: "Julia",
            aliases: [
                "jl"
            ],
            import: ()=>u(()=>import("./julia-BBuGR-5E.js"), __vite__mapDeps([59,21,22,23,24,14,18,2,60]))
        },
        {
            id: "kotlin",
            name: "Kotlin",
            aliases: [
                "kt",
                "kts"
            ],
            import: ()=>u(()=>import("./kotlin-B5lbUyaz.js"), [])
        },
        {
            id: "kusto",
            name: "Kusto",
            aliases: [
                "kql"
            ],
            import: ()=>u(()=>import("./kusto-mebxcVVE.js"), [])
        },
        {
            id: "latex",
            name: "LaTeX",
            import: ()=>u(()=>import("./latex-C-cWTeAZ.js"), __vite__mapDeps([61,62,60]))
        },
        {
            id: "lean",
            name: "Lean 4",
            aliases: [
                "lean4"
            ],
            import: ()=>u(()=>import("./lean-XBlWyCtg.js"), [])
        },
        {
            id: "less",
            name: "Less",
            import: ()=>u(()=>import("./less-BfCpw3nA.js"), [])
        },
        {
            id: "liquid",
            name: "Liquid",
            import: ()=>u(()=>import("./liquid-D3W5UaiH.js"), __vite__mapDeps([63,1,2,3,9]))
        },
        {
            id: "log",
            name: "Log file",
            import: ()=>u(()=>import("./log-Cc5clBb7.js"), [])
        },
        {
            id: "logo",
            name: "Logo",
            import: ()=>u(()=>import("./logo-IuBKFhSY.js"), [])
        },
        {
            id: "lua",
            name: "Lua",
            import: ()=>u(()=>import("./lua-CvWAzNxB.js"), __vite__mapDeps([37,24]))
        },
        {
            id: "luau",
            name: "Luau",
            import: ()=>u(()=>import("./luau-Du5NY7AG.js"), [])
        },
        {
            id: "make",
            name: "Makefile",
            aliases: [
                "makefile"
            ],
            import: ()=>u(()=>import("./make-Bvotw-X0.js"), [])
        },
        {
            id: "markdown",
            name: "Markdown",
            aliases: [
                "md"
            ],
            import: ()=>u(()=>import("./markdown-UIAJJxZW.js"), [])
        },
        {
            id: "marko",
            name: "Marko",
            import: ()=>u(()=>import("./marko-z0MBrx5-.js"), __vite__mapDeps([64,3,65,5,2]))
        },
        {
            id: "matlab",
            name: "MATLAB",
            import: ()=>u(()=>import("./matlab-D9-PGadD.js"), [])
        },
        {
            id: "mdc",
            name: "MDC",
            import: ()=>u(()=>import("./mdc-DB_EDNY_.js"), __vite__mapDeps([66,42,38,28,1,2,3]))
        },
        {
            id: "mdx",
            name: "MDX",
            import: ()=>u(()=>import("./mdx-sdHcTMYB.js"), [])
        },
        {
            id: "mermaid",
            name: "Mermaid",
            aliases: [
                "mmd"
            ],
            import: ()=>u(()=>import("./mermaid-Ci6OQyBP.js"), [])
        },
        {
            id: "mipsasm",
            name: "MIPS Assembly",
            aliases: [
                "mips"
            ],
            import: ()=>u(()=>import("./mipsasm-BC5c_5Pe.js"), [])
        },
        {
            id: "mojo",
            name: "Mojo",
            import: ()=>u(()=>import("./mojo-Tz6hzZYG.js"), [])
        },
        {
            id: "move",
            name: "Move",
            import: ()=>u(()=>import("./move-DB_GagMm.js"), [])
        },
        {
            id: "narrat",
            name: "Narrat Language",
            aliases: [
                "nar"
            ],
            import: ()=>u(()=>import("./narrat-DLbgOhZU.js"), [])
        },
        {
            id: "nextflow",
            name: "Nextflow",
            aliases: [
                "nf"
            ],
            import: ()=>u(()=>import("./nextflow-B0XVJmRM.js"), [])
        },
        {
            id: "nginx",
            name: "Nginx",
            import: ()=>u(()=>import("./nginx-D_VnBJ67.js"), __vite__mapDeps([67,37,24]))
        },
        {
            id: "nim",
            name: "Nim",
            import: ()=>u(()=>import("./nim-ZlGxZxc3.js"), __vite__mapDeps([68,24,1,2,3,7,8,23,42]))
        },
        {
            id: "nix",
            name: "Nix",
            import: ()=>u(()=>import("./nix-shcSOmrb.js"), [])
        },
        {
            id: "nushell",
            name: "nushell",
            aliases: [
                "nu"
            ],
            import: ()=>u(()=>import("./nushell-D4Tzg5kh.js"), [])
        },
        {
            id: "objective-c",
            name: "Objective-C",
            aliases: [
                "objc"
            ],
            import: ()=>u(()=>import("./objective-c-Deuh7S70.js"), [])
        },
        {
            id: "objective-cpp",
            name: "Objective-C++",
            import: ()=>u(()=>import("./objective-cpp-BUEGK8hf.js"), [])
        },
        {
            id: "ocaml",
            name: "OCaml",
            import: ()=>u(()=>import("./ocaml-BNioltXt.js"), [])
        },
        {
            id: "pascal",
            name: "Pascal",
            import: ()=>u(()=>import("./pascal-JqZropPD.js"), [])
        },
        {
            id: "perl",
            name: "Perl",
            import: ()=>u(()=>import("./perl-CHQXSrWU.js"), __vite__mapDeps([69,1,2,3,7,8,14]))
        },
        {
            id: "php",
            name: "PHP",
            import: ()=>u(()=>import("./php-B5ebYQev.js"), __vite__mapDeps([70,1,2,3,7,8,14,9]))
        },
        {
            id: "plsql",
            name: "PL/SQL",
            import: ()=>u(()=>import("./plsql-LKU2TuZ1.js"), [])
        },
        {
            id: "po",
            name: "Gettext PO",
            aliases: [
                "pot",
                "potx"
            ],
            import: ()=>u(()=>import("./po-BFLt1xDp.js"), [])
        },
        {
            id: "polar",
            name: "Polar",
            import: ()=>u(()=>import("./polar-DKykz6zU.js"), [])
        },
        {
            id: "postcss",
            name: "PostCSS",
            import: ()=>u(()=>import("./postcss-B3ZDOciz.js"), [])
        },
        {
            id: "powerquery",
            name: "PowerQuery",
            import: ()=>u(()=>import("./powerquery-CSHBycmS.js"), [])
        },
        {
            id: "powershell",
            name: "PowerShell",
            aliases: [
                "ps",
                "ps1"
            ],
            import: ()=>u(()=>import("./powershell-BIEUsx6d.js"), [])
        },
        {
            id: "prisma",
            name: "Prisma",
            import: ()=>u(()=>import("./prisma-B48N-Iqd.js"), [])
        },
        {
            id: "prolog",
            name: "Prolog",
            import: ()=>u(()=>import("./prolog-BY-TUvya.js"), [])
        },
        {
            id: "proto",
            name: "Protocol Buffer 3",
            aliases: [
                "protobuf"
            ],
            import: ()=>u(()=>import("./proto-zocC4JxJ.js"), [])
        },
        {
            id: "pug",
            name: "Pug",
            aliases: [
                "jade"
            ],
            import: ()=>u(()=>import("./pug-CM9l7STV.js"), __vite__mapDeps([71,2,3,1]))
        },
        {
            id: "puppet",
            name: "Puppet",
            import: ()=>u(()=>import("./puppet-Cza_XSSt.js"), [])
        },
        {
            id: "purescript",
            name: "PureScript",
            import: ()=>u(()=>import("./purescript-Bg-kzb6g.js"), [])
        },
        {
            id: "python",
            name: "Python",
            aliases: [
                "py"
            ],
            import: ()=>u(()=>import("./python-DhUJRlN_.js"), [])
        },
        {
            id: "qml",
            name: "QML",
            import: ()=>u(()=>import("./qml-D8XfuvdV.js"), __vite__mapDeps([72,2]))
        },
        {
            id: "qmldir",
            name: "QML Directory",
            import: ()=>u(()=>import("./qmldir-C8lEn-DE.js"), [])
        },
        {
            id: "qss",
            name: "Qt Style Sheets",
            import: ()=>u(()=>import("./qss-DhMKtDLN.js"), [])
        },
        {
            id: "r",
            name: "R",
            import: ()=>u(()=>import("./r-CwjWoCRV.js"), [])
        },
        {
            id: "racket",
            name: "Racket",
            import: ()=>u(()=>import("./racket-CzouJOBO.js"), [])
        },
        {
            id: "raku",
            name: "Raku",
            aliases: [
                "perl6"
            ],
            import: ()=>u(()=>import("./raku-B1bQXN8T.js"), [])
        },
        {
            id: "razor",
            name: "ASP.NET Razor",
            import: ()=>u(()=>import("./razor-CNLDkMZG.js"), __vite__mapDeps([73,1,2,3,74]))
        },
        {
            id: "reg",
            name: "Windows Registry Script",
            import: ()=>u(()=>import("./reg-5LuOXUq_.js"), [])
        },
        {
            id: "regexp",
            name: "RegExp",
            aliases: [
                "regex"
            ],
            import: ()=>u(()=>import("./regexp-DWJ3fJO_.js"), [])
        },
        {
            id: "rel",
            name: "Rel",
            import: ()=>u(()=>import("./rel-DJlmqQ1C.js"), [])
        },
        {
            id: "riscv",
            name: "RISC-V",
            import: ()=>u(()=>import("./riscv-QhoSD0DR.js"), [])
        },
        {
            id: "rst",
            name: "reStructuredText",
            import: ()=>u(()=>import("./rst-4NLicBqY.js"), __vite__mapDeps([75,28,1,2,3,21,22,23,24,14,18,26,38,76,32,33,7,8,34,11,35,36,37]))
        },
        {
            id: "ruby",
            name: "Ruby",
            aliases: [
                "rb"
            ],
            import: ()=>u(()=>import("./ruby-DeZ3UC14.js"), __vite__mapDeps([32,1,2,3,33,7,8,14,34,11,35,36,21,22,23,24,26,37,38]))
        },
        {
            id: "rust",
            name: "Rust",
            aliases: [
                "rs"
            ],
            import: ()=>u(()=>import("./rust-Be6lgOlo.js"), [])
        },
        {
            id: "sas",
            name: "SAS",
            import: ()=>u(()=>import("./sas-BmTFh92c.js"), __vite__mapDeps([77,14]))
        },
        {
            id: "sass",
            name: "Sass",
            import: ()=>u(()=>import("./sass-BJ4Li9vH.js"), [])
        },
        {
            id: "scala",
            name: "Scala",
            import: ()=>u(()=>import("./scala-DQVVAn-B.js"), [])
        },
        {
            id: "scheme",
            name: "Scheme",
            import: ()=>u(()=>import("./scheme-BJGe-b2p.js"), [])
        },
        {
            id: "scss",
            name: "SCSS",
            import: ()=>u(()=>import("./scss-C31hgJw-.js"), __vite__mapDeps([5,3]))
        },
        {
            id: "sdbl",
            name: "1C (Query)",
            aliases: [
                "1c-query"
            ],
            import: ()=>u(()=>import("./sdbl-BLhTXw86.js"), [])
        },
        {
            id: "shaderlab",
            name: "ShaderLab",
            aliases: [
                "shader"
            ],
            import: ()=>u(()=>import("./shaderlab-B7qAK45m.js"), __vite__mapDeps([78,79]))
        },
        {
            id: "shellscript",
            name: "Shell",
            aliases: [
                "bash",
                "sh",
                "shell",
                "zsh"
            ],
            import: ()=>u(()=>import("./shellscript-atvbtKCR.js"), [])
        },
        {
            id: "shellsession",
            name: "Shell Session",
            aliases: [
                "console"
            ],
            import: ()=>u(()=>import("./shellsession-C_rIy8kc.js"), __vite__mapDeps([80,26]))
        },
        {
            id: "smalltalk",
            name: "Smalltalk",
            import: ()=>u(()=>import("./smalltalk-DkLiglaE.js"), [])
        },
        {
            id: "solidity",
            name: "Solidity",
            import: ()=>u(()=>import("./solidity-C1w2a3ep.js"), [])
        },
        {
            id: "soy",
            name: "Closure Templates",
            aliases: [
                "closure-templates"
            ],
            import: ()=>u(()=>import("./soy-C-lX7w71.js"), __vite__mapDeps([81,1,2,3]))
        },
        {
            id: "sparql",
            name: "SPARQL",
            import: ()=>u(()=>import("./sparql-bYkjHRlG.js"), __vite__mapDeps([82,83]))
        },
        {
            id: "splunk",
            name: "Splunk Query Language",
            aliases: [
                "spl"
            ],
            import: ()=>u(()=>import("./splunk-Cf8iN4DR.js"), [])
        },
        {
            id: "sql",
            name: "SQL",
            import: ()=>u(()=>import("./sql-COK4E0Yg.js"), [])
        },
        {
            id: "ssh-config",
            name: "SSH Config",
            import: ()=>u(()=>import("./ssh-config-BknIz3MU.js"), [])
        },
        {
            id: "stata",
            name: "Stata",
            import: ()=>u(()=>import("./stata-DorPZHa4.js"), __vite__mapDeps([84,14]))
        },
        {
            id: "stylus",
            name: "Stylus",
            aliases: [
                "styl"
            ],
            import: ()=>u(()=>import("./stylus-BeQkCIfX.js"), [])
        },
        {
            id: "svelte",
            name: "Svelte",
            import: ()=>u(()=>import("./svelte-MSaWC3Je.js"), __vite__mapDeps([85,2,11,3,12]))
        },
        {
            id: "swift",
            name: "Swift",
            import: ()=>u(()=>import("./swift-BSxZ-RaX.js"), [])
        },
        {
            id: "system-verilog",
            name: "SystemVerilog",
            import: ()=>u(()=>import("./system-verilog-C7L56vO4.js"), [])
        },
        {
            id: "systemd",
            name: "Systemd Units",
            import: ()=>u(()=>import("./systemd-CUnW07Te.js"), [])
        },
        {
            id: "talonscript",
            name: "TalonScript",
            aliases: [
                "talon"
            ],
            import: ()=>u(()=>import("./talonscript-C1XDQQGZ.js"), [])
        },
        {
            id: "tasl",
            name: "Tasl",
            import: ()=>u(()=>import("./tasl-CQjiPCtT.js"), [])
        },
        {
            id: "tcl",
            name: "Tcl",
            import: ()=>u(()=>import("./tcl-DQ1-QYvQ.js"), [])
        },
        {
            id: "templ",
            name: "Templ",
            import: ()=>u(()=>import("./templ-dwX3ZSMB.js"), __vite__mapDeps([86,87,2,3]))
        },
        {
            id: "terraform",
            name: "Terraform",
            aliases: [
                "tf",
                "tfvars"
            ],
            import: ()=>u(()=>import("./terraform-BbSNqyBO.js"), [])
        },
        {
            id: "tex",
            name: "TeX",
            import: ()=>u(()=>import("./tex-rYs2v40G.js"), __vite__mapDeps([62,60]))
        },
        {
            id: "toml",
            name: "TOML",
            import: ()=>u(()=>import("./toml-CB2ApiWb.js"), [])
        },
        {
            id: "ts-tags",
            name: "TypeScript with Tags",
            aliases: [
                "lit"
            ],
            import: ()=>u(()=>import("./ts-tags-CipyTH0X.js"), __vite__mapDeps([88,11,3,2,23,24,1,14,7,8]))
        },
        {
            id: "tsv",
            name: "TSV",
            import: ()=>u(()=>import("./tsv-B_m7g4N7.js"), [])
        },
        {
            id: "tsx",
            name: "TSX",
            import: ()=>u(()=>import("./tsx-B6W0miNI.js"), [])
        },
        {
            id: "turtle",
            name: "Turtle",
            import: ()=>u(()=>import("./turtle-BMR_PYu6.js"), [])
        },
        {
            id: "twig",
            name: "Twig",
            import: ()=>u(()=>import("./twig-NC5TFiHP.js"), __vite__mapDeps([89,3,2,5,70,1,7,8,14,9,18,32,33,34,11,35,36,21,22,23,24,26,37,38]))
        },
        {
            id: "typescript",
            name: "TypeScript",
            aliases: [
                "ts"
            ],
            import: ()=>u(()=>import("./typescript-Dj6nwHGl.js"), [])
        },
        {
            id: "typespec",
            name: "TypeSpec",
            aliases: [
                "tsp"
            ],
            import: ()=>u(()=>import("./typespec-BpWG_bgh.js"), [])
        },
        {
            id: "typst",
            name: "Typst",
            aliases: [
                "typ"
            ],
            import: ()=>u(()=>import("./typst-BVUVsWT6.js"), [])
        },
        {
            id: "v",
            name: "V",
            import: ()=>u(()=>import("./v-CAQ2eGtk.js"), [])
        },
        {
            id: "vala",
            name: "Vala",
            import: ()=>u(()=>import("./vala-BFOHcciG.js"), [])
        },
        {
            id: "vb",
            name: "Visual Basic",
            aliases: [
                "cmd"
            ],
            import: ()=>u(()=>import("./vb-CdO5JTpU.js"), [])
        },
        {
            id: "verilog",
            name: "Verilog",
            import: ()=>u(()=>import("./verilog-CJaU5se_.js"), [])
        },
        {
            id: "vhdl",
            name: "VHDL",
            import: ()=>u(()=>import("./vhdl-DYoNaHQp.js"), [])
        },
        {
            id: "viml",
            name: "Vim Script",
            aliases: [
                "vim",
                "vimscript"
            ],
            import: ()=>u(()=>import("./viml-m4uW47V2.js"), [])
        },
        {
            id: "vue",
            name: "Vue",
            import: ()=>u(()=>import("./vue-BuYVFjOK.js"), __vite__mapDeps([90,1,2,3,11,9,28]))
        },
        {
            id: "vue-html",
            name: "Vue HTML",
            import: ()=>u(()=>import("./vue-html-xdeiXROB.js"), __vite__mapDeps([91,90,1,2,3,11,9,28]))
        },
        {
            id: "vyper",
            name: "Vyper",
            aliases: [
                "vy"
            ],
            import: ()=>u(()=>import("./vyper-nyqBNV6O.js"), [])
        },
        {
            id: "wasm",
            name: "WebAssembly",
            import: ()=>u(()=>import("./wasm-C6j12Q_x.js"), [])
        },
        {
            id: "wenyan",
            name: "Wenyan",
            aliases: [
                "文言"
            ],
            import: ()=>u(()=>import("./wenyan-7A4Fjokl.js"), [])
        },
        {
            id: "wgsl",
            name: "WGSL",
            import: ()=>u(()=>import("./wgsl-CB0Krxn9.js"), [])
        },
        {
            id: "wikitext",
            name: "Wikitext",
            aliases: [
                "mediawiki",
                "wiki"
            ],
            import: ()=>u(()=>import("./wikitext-DCE3LsBG.js"), [])
        },
        {
            id: "wolfram",
            name: "Wolfram",
            aliases: [
                "wl"
            ],
            import: ()=>u(()=>import("./wolfram-C3FkfJm5.js"), [])
        },
        {
            id: "xml",
            name: "XML",
            import: ()=>u(()=>import("./xml-e3z08dGr.js"), __vite__mapDeps([7,8]))
        },
        {
            id: "xsl",
            name: "XSL",
            import: ()=>u(()=>import("./xsl-Dd0NUgwM.js"), __vite__mapDeps([92,7,8]))
        },
        {
            id: "yaml",
            name: "YAML",
            aliases: [
                "yml"
            ],
            import: ()=>u(()=>import("./yaml-CVw76BM1.js"), [])
        },
        {
            id: "zenscript",
            name: "ZenScript",
            import: ()=>u(()=>import("./zenscript-HnGAYVZD.js"), [])
        },
        {
            id: "zig",
            name: "Zig",
            import: ()=>u(()=>import("./zig-BVz_zdnA.js"), [])
        }
    ], Ts = Object.fromEntries(hr.map((t)=>[
            t.id,
            t.import
        ])), Rs = Object.fromEntries(hr.flatMap((t)=>t.aliases?.map((e)=>[
                e,
                t.import
            ]) || [])), Ls = {
        ...Ts,
        ...Rs
    }, Ps = [
        {
            id: "andromeeda",
            displayName: "Andromeeda",
            type: "dark",
            import: ()=>u(()=>import("./andromeeda-C3khCPGq.js"), [])
        },
        {
            id: "aurora-x",
            displayName: "Aurora X",
            type: "dark",
            import: ()=>u(()=>import("./aurora-x-D-2ljcwZ.js"), [])
        },
        {
            id: "ayu-dark",
            displayName: "Ayu Dark",
            type: "dark",
            import: ()=>u(()=>import("./ayu-dark-Cv9koXgw.js"), [])
        },
        {
            id: "catppuccin-frappe",
            displayName: "Catppuccin Frappé",
            type: "dark",
            import: ()=>u(()=>import("./catppuccin-frappe-CD_QflpE.js"), [])
        },
        {
            id: "catppuccin-latte",
            displayName: "Catppuccin Latte",
            type: "light",
            import: ()=>u(()=>import("./catppuccin-latte-DRW-0cLl.js"), [])
        },
        {
            id: "catppuccin-macchiato",
            displayName: "Catppuccin Macchiato",
            type: "dark",
            import: ()=>u(()=>import("./catppuccin-macchiato-C-_shW-Y.js"), [])
        },
        {
            id: "catppuccin-mocha",
            displayName: "Catppuccin Mocha",
            type: "dark",
            import: ()=>u(()=>import("./catppuccin-mocha-LGGdnPYs.js"), [])
        },
        {
            id: "dark-plus",
            displayName: "Dark Plus",
            type: "dark",
            import: ()=>u(()=>import("./dark-plus-C3mMm8J8.js"), [])
        },
        {
            id: "dracula",
            displayName: "Dracula Theme",
            type: "dark",
            import: ()=>u(()=>import("./dracula-BzJJZx-M.js"), [])
        },
        {
            id: "dracula-soft",
            displayName: "Dracula Theme Soft",
            type: "dark",
            import: ()=>u(()=>import("./dracula-soft-BXkSAIEj.js"), [])
        },
        {
            id: "everforest-dark",
            displayName: "Everforest Dark",
            type: "dark",
            import: ()=>u(()=>import("./everforest-dark-BgDCqdQA.js"), [])
        },
        {
            id: "everforest-light",
            displayName: "Everforest Light",
            type: "light",
            import: ()=>u(()=>import("./everforest-light-C8M2exoo.js"), [])
        },
        {
            id: "github-dark",
            displayName: "GitHub Dark",
            type: "dark",
            import: ()=>u(()=>import("./github-dark-DHJKELXO.js"), [])
        },
        {
            id: "github-dark-default",
            displayName: "GitHub Dark Default",
            type: "dark",
            import: ()=>u(()=>import("./github-dark-default-Cuk6v7N8.js"), [])
        },
        {
            id: "github-dark-dimmed",
            displayName: "GitHub Dark Dimmed",
            type: "dark",
            import: ()=>u(()=>import("./github-dark-dimmed-DH5Ifo-i.js"), [])
        },
        {
            id: "github-dark-high-contrast",
            displayName: "GitHub Dark High Contrast",
            type: "dark",
            import: ()=>u(()=>import("./github-dark-high-contrast-E3gJ1_iC.js"), [])
        },
        {
            id: "github-light",
            displayName: "GitHub Light",
            type: "light",
            import: ()=>u(()=>import("./github-light-DAi9KRSo.js"), [])
        },
        {
            id: "github-light-default",
            displayName: "GitHub Light Default",
            type: "light",
            import: ()=>u(()=>import("./github-light-default-D7oLnXFd.js"), [])
        },
        {
            id: "github-light-high-contrast",
            displayName: "GitHub Light High Contrast",
            type: "light",
            import: ()=>u(()=>import("./github-light-high-contrast-BfjtVDDH.js"), [])
        },
        {
            id: "houston",
            displayName: "Houston",
            type: "dark",
            import: ()=>u(()=>import("./houston-DnULxvSX.js"), [])
        },
        {
            id: "kanagawa-dragon",
            displayName: "Kanagawa Dragon",
            type: "dark",
            import: ()=>u(()=>import("./kanagawa-dragon-CkXjmgJE.js"), [])
        },
        {
            id: "kanagawa-lotus",
            displayName: "Kanagawa Lotus",
            type: "light",
            import: ()=>u(()=>import("./kanagawa-lotus-CfQXZHmo.js"), [])
        },
        {
            id: "kanagawa-wave",
            displayName: "Kanagawa Wave",
            type: "dark",
            import: ()=>u(()=>import("./kanagawa-wave-DWedfzmr.js"), [])
        },
        {
            id: "laserwave",
            displayName: "LaserWave",
            type: "dark",
            import: ()=>u(()=>import("./laserwave-DUszq2jm.js"), [])
        },
        {
            id: "light-plus",
            displayName: "Light Plus",
            type: "light",
            import: ()=>u(()=>import("./light-plus-B7mTdjB0.js"), [])
        },
        {
            id: "material-theme",
            displayName: "Material Theme",
            type: "dark",
            import: ()=>u(()=>import("./material-theme-D5KoaKCx.js"), [])
        },
        {
            id: "material-theme-darker",
            displayName: "Material Theme Darker",
            type: "dark",
            import: ()=>u(()=>import("./material-theme-darker-BfHTSMKl.js"), [])
        },
        {
            id: "material-theme-lighter",
            displayName: "Material Theme Lighter",
            type: "light",
            import: ()=>u(()=>import("./material-theme-lighter-B0m2ddpp.js"), [])
        },
        {
            id: "material-theme-ocean",
            displayName: "Material Theme Ocean",
            type: "dark",
            import: ()=>u(()=>import("./material-theme-ocean-CyktbL80.js"), [])
        },
        {
            id: "material-theme-palenight",
            displayName: "Material Theme Palenight",
            type: "dark",
            import: ()=>u(()=>import("./material-theme-palenight-Csfq5Kiy.js"), [])
        },
        {
            id: "min-dark",
            displayName: "Min Dark",
            type: "dark",
            import: ()=>u(()=>import("./min-dark-CafNBF8u.js"), [])
        },
        {
            id: "min-light",
            displayName: "Min Light",
            type: "light",
            import: ()=>u(()=>import("./min-light-CTRr51gU.js"), [])
        },
        {
            id: "monokai",
            displayName: "Monokai",
            type: "dark",
            import: ()=>u(()=>import("./monokai-D4h5O-jR.js"), [])
        },
        {
            id: "night-owl",
            displayName: "Night Owl",
            type: "dark",
            import: ()=>u(()=>import("./night-owl-C39BiMTA.js"), [])
        },
        {
            id: "nord",
            displayName: "Nord",
            type: "dark",
            import: ()=>u(()=>import("./nord-Ddv68eIx.js"), [])
        },
        {
            id: "one-dark-pro",
            displayName: "One Dark Pro",
            type: "dark",
            import: ()=>u(()=>import("./one-dark-pro-GBQ2dnAY.js"), [])
        },
        {
            id: "one-light",
            displayName: "One Light",
            type: "light",
            import: ()=>u(()=>import("./one-light-PoHY5YXO.js"), [])
        },
        {
            id: "plastic",
            displayName: "Plastic",
            type: "dark",
            import: ()=>u(()=>import("./plastic-3e1v2bzS.js"), [])
        },
        {
            id: "poimandres",
            displayName: "Poimandres",
            type: "dark",
            import: ()=>u(()=>import("./poimandres-CS3Unz2-.js"), [])
        },
        {
            id: "red",
            displayName: "Red",
            type: "dark",
            import: ()=>u(()=>import("./red-bN70gL4F.js"), [])
        },
        {
            id: "rose-pine",
            displayName: "Rosé Pine",
            type: "dark",
            import: ()=>u(()=>import("./rose-pine-CmCqftbK.js"), [])
        },
        {
            id: "rose-pine-dawn",
            displayName: "Rosé Pine Dawn",
            type: "light",
            import: ()=>u(()=>import("./rose-pine-dawn-Ds-gbosJ.js"), [])
        },
        {
            id: "rose-pine-moon",
            displayName: "Rosé Pine Moon",
            type: "dark",
            import: ()=>u(()=>import("./rose-pine-moon-CjDtw9vr.js"), [])
        },
        {
            id: "slack-dark",
            displayName: "Slack Dark",
            type: "dark",
            import: ()=>u(()=>import("./slack-dark-BthQWCQV.js"), [])
        },
        {
            id: "slack-ochin",
            displayName: "Slack Ochin",
            type: "light",
            import: ()=>u(()=>import("./slack-ochin-DqwNpetd.js"), [])
        },
        {
            id: "snazzy-light",
            displayName: "Snazzy Light",
            type: "light",
            import: ()=>u(()=>import("./snazzy-light-Bw305WKR.js"), [])
        },
        {
            id: "solarized-dark",
            displayName: "Solarized Dark",
            type: "dark",
            import: ()=>u(()=>import("./solarized-dark-DXbdFlpD.js"), [])
        },
        {
            id: "solarized-light",
            displayName: "Solarized Light",
            type: "light",
            import: ()=>u(()=>import("./solarized-light-L9t79GZl.js"), [])
        },
        {
            id: "synthwave-84",
            displayName: "Synthwave '84",
            type: "dark",
            import: ()=>u(()=>import("./synthwave-84-CbfX1IO0.js"), [])
        },
        {
            id: "tokyo-night",
            displayName: "Tokyo Night",
            type: "dark",
            import: ()=>u(()=>import("./tokyo-night-DBQeEorK.js"), [])
        },
        {
            id: "vesper",
            displayName: "Vesper",
            type: "dark",
            import: ()=>u(()=>import("./vesper-BEBZ7ncR.js"), [])
        },
        {
            id: "vitesse-black",
            displayName: "Vitesse Black",
            type: "dark",
            import: ()=>u(()=>import("./vitesse-black-Bkuqu6BP.js"), [])
        },
        {
            id: "vitesse-dark",
            displayName: "Vitesse Dark",
            type: "dark",
            import: ()=>u(()=>import("./vitesse-dark-D0r3Knsf.js"), [])
        },
        {
            id: "vitesse-light",
            displayName: "Vitesse Light",
            type: "light",
            import: ()=>u(()=>import("./vitesse-light-CVO1_9PV.js"), [])
        }
    ], ks = Object.fromEntries(Ps.map((t)=>[
            t.id,
            t.import
        ]));
    let ce = class extends Error {
        constructor(e){
            super(e), this.name = "ShikiError";
        }
    }, ln = class extends Error {
        constructor(e){
            super(e), this.name = "ShikiError";
        }
    };
    function Cs() {
        return 2147483648;
    }
    function Is() {
        return typeof performance < "u" ? performance.now() : Date.now();
    }
    const Os = (t, e)=>t + (e - t % e) % e;
    async function xs(t) {
        let e, n;
        const r = {};
        function i(f) {
            n = f, r.HEAPU8 = new Uint8Array(f), r.HEAPU32 = new Uint32Array(f);
        }
        function o(f, h, E) {
            r.HEAPU8.copyWithin(f, h, h + E);
        }
        function s(f) {
            try {
                return e.grow(f - n.byteLength + 65535 >>> 16), i(e.buffer), 1;
            } catch  {}
        }
        function a(f) {
            const h = r.HEAPU8.length;
            f = f >>> 0;
            const E = Cs();
            if (f > E) return !1;
            for(let g = 1; g <= 4; g *= 2){
                let w = h * (1 + .2 / g);
                w = Math.min(w, f + 100663296);
                const m = Math.min(E, Os(Math.max(f, w), 65536));
                if (s(m)) return !0;
            }
            return !1;
        }
        const l = typeof TextDecoder < "u" ? new TextDecoder("utf8") : void 0;
        function c(f, h, E = 1024) {
            const g = h + E;
            let w = h;
            for(; f[w] && !(w >= g);)++w;
            if (w - h > 16 && f.buffer && l) return l.decode(f.subarray(h, w));
            let m = "";
            for(; h < w;){
                let y = f[h++];
                if (!(y & 128)) {
                    m += String.fromCharCode(y);
                    continue;
                }
                const v = f[h++] & 63;
                if ((y & 224) === 192) {
                    m += String.fromCharCode((y & 31) << 6 | v);
                    continue;
                }
                const R = f[h++] & 63;
                if ((y & 240) === 224 ? y = (y & 15) << 12 | v << 6 | R : y = (y & 7) << 18 | v << 12 | R << 6 | f[h++] & 63, y < 65536) m += String.fromCharCode(y);
                else {
                    const O = y - 65536;
                    m += String.fromCharCode(55296 | O >> 10, 56320 | O & 1023);
                }
            }
            return m;
        }
        function d(f, h) {
            return f ? c(r.HEAPU8, f, h) : "";
        }
        const _ = {
            emscripten_get_now: Is,
            emscripten_memcpy_big: o,
            emscripten_resize_heap: a,
            fd_write: ()=>0
        };
        async function p() {
            const h = await t({
                env: _,
                wasi_snapshot_preview1: _
            });
            e = h.memory, i(e.buffer), Object.assign(r, h), r.UTF8ToString = d;
        }
        return await p(), r;
    }
    var Ds = Object.defineProperty, Ns = (t, e, n)=>e in t ? Ds(t, e, {
            enumerable: !0,
            configurable: !0,
            writable: !0,
            value: n
        }) : t[e] = n, $ = (t, e, n)=>(Ns(t, typeof e != "symbol" ? e + "" : e, n), n);
    let G = null;
    function Vs(t) {
        throw new ln(t.UTF8ToString(t.getLastOnigError()));
    }
    class Pt {
        constructor(e){
            $(this, "utf16Length"), $(this, "utf8Length"), $(this, "utf16Value"), $(this, "utf8Value"), $(this, "utf16OffsetToUtf8"), $(this, "utf8OffsetToUtf16");
            const n = e.length, r = Pt._utf8ByteLength(e), i = r !== n, o = i ? new Uint32Array(n + 1) : null;
            i && (o[n] = r);
            const s = i ? new Uint32Array(r + 1) : null;
            i && (s[r] = n);
            const a = new Uint8Array(r);
            let l = 0;
            for(let c = 0; c < n; c++){
                const d = e.charCodeAt(c);
                let _ = d, p = !1;
                if (d >= 55296 && d <= 56319 && c + 1 < n) {
                    const f = e.charCodeAt(c + 1);
                    f >= 56320 && f <= 57343 && (_ = (d - 55296 << 10) + 65536 | f - 56320, p = !0);
                }
                i && (o[c] = l, p && (o[c + 1] = l), _ <= 127 ? s[l + 0] = c : _ <= 2047 ? (s[l + 0] = c, s[l + 1] = c) : _ <= 65535 ? (s[l + 0] = c, s[l + 1] = c, s[l + 2] = c) : (s[l + 0] = c, s[l + 1] = c, s[l + 2] = c, s[l + 3] = c)), _ <= 127 ? a[l++] = _ : _ <= 2047 ? (a[l++] = 192 | (_ & 1984) >>> 6, a[l++] = 128 | (_ & 63) >>> 0) : _ <= 65535 ? (a[l++] = 224 | (_ & 61440) >>> 12, a[l++] = 128 | (_ & 4032) >>> 6, a[l++] = 128 | (_ & 63) >>> 0) : (a[l++] = 240 | (_ & 1835008) >>> 18, a[l++] = 128 | (_ & 258048) >>> 12, a[l++] = 128 | (_ & 4032) >>> 6, a[l++] = 128 | (_ & 63) >>> 0), p && c++;
            }
            this.utf16Length = n, this.utf8Length = r, this.utf16Value = e, this.utf8Value = a, this.utf16OffsetToUtf8 = o, this.utf8OffsetToUtf16 = s;
        }
        static _utf8ByteLength(e) {
            let n = 0;
            for(let r = 0, i = e.length; r < i; r++){
                const o = e.charCodeAt(r);
                let s = o, a = !1;
                if (o >= 55296 && o <= 56319 && r + 1 < i) {
                    const l = e.charCodeAt(r + 1);
                    l >= 56320 && l <= 57343 && (s = (o - 55296 << 10) + 65536 | l - 56320, a = !0);
                }
                s <= 127 ? n += 1 : s <= 2047 ? n += 2 : s <= 65535 ? n += 3 : n += 4, a && r++;
            }
            return n;
        }
        createString(e) {
            const n = e.omalloc(this.utf8Length);
            return e.HEAPU8.set(this.utf8Value, n), n;
        }
    }
    const ne = class {
        constructor(t){
            if ($(this, "id", ++ne.LAST_ID), $(this, "_onigBinding"), $(this, "content"), $(this, "utf16Length"), $(this, "utf8Length"), $(this, "utf16OffsetToUtf8"), $(this, "utf8OffsetToUtf16"), $(this, "ptr"), !G) throw new ln("Must invoke loadWasm first.");
            this._onigBinding = G, this.content = t;
            const e = new Pt(t);
            this.utf16Length = e.utf16Length, this.utf8Length = e.utf8Length, this.utf16OffsetToUtf8 = e.utf16OffsetToUtf8, this.utf8OffsetToUtf16 = e.utf8OffsetToUtf16, this.utf8Length < 1e4 && !ne._sharedPtrInUse ? (ne._sharedPtr || (ne._sharedPtr = G.omalloc(1e4)), ne._sharedPtrInUse = !0, G.HEAPU8.set(e.utf8Value, ne._sharedPtr), this.ptr = ne._sharedPtr) : this.ptr = e.createString(G);
        }
        convertUtf8OffsetToUtf16(t) {
            return this.utf8OffsetToUtf16 ? t < 0 ? 0 : t > this.utf8Length ? this.utf16Length : this.utf8OffsetToUtf16[t] : t;
        }
        convertUtf16OffsetToUtf8(t) {
            return this.utf16OffsetToUtf8 ? t < 0 ? 0 : t > this.utf16Length ? this.utf8Length : this.utf16OffsetToUtf8[t] : t;
        }
        dispose() {
            this.ptr === ne._sharedPtr ? ne._sharedPtrInUse = !1 : this._onigBinding.ofree(this.ptr);
        }
    };
    let Je = ne;
    $(Je, "LAST_ID", 0);
    $(Je, "_sharedPtr", 0);
    $(Je, "_sharedPtrInUse", !1);
    class Ms {
        constructor(e){
            if ($(this, "_onigBinding"), $(this, "_ptr"), !G) throw new ln("Must invoke loadWasm first.");
            const n = [], r = [];
            for(let a = 0, l = e.length; a < l; a++){
                const c = new Pt(e[a]);
                n[a] = c.createString(G), r[a] = c.utf8Length;
            }
            const i = G.omalloc(4 * e.length);
            G.HEAPU32.set(n, i / 4);
            const o = G.omalloc(4 * e.length);
            G.HEAPU32.set(r, o / 4);
            const s = G.createOnigScanner(i, o, e.length);
            for(let a = 0, l = e.length; a < l; a++)G.ofree(n[a]);
            G.ofree(o), G.ofree(i), s === 0 && Vs(G), this._onigBinding = G, this._ptr = s;
        }
        dispose() {
            this._onigBinding.freeOnigScanner(this._ptr);
        }
        findNextMatchSync(e, n, r) {
            let i = 0;
            if (typeof r == "number" && (i = r), typeof e == "string") {
                e = new Je(e);
                const o = this._findNextMatchSync(e, n, !1, i);
                return e.dispose(), o;
            }
            return this._findNextMatchSync(e, n, !1, i);
        }
        _findNextMatchSync(e, n, r, i) {
            const o = this._onigBinding, s = o.findNextOnigScannerMatch(this._ptr, e.id, e.ptr, e.utf8Length, e.convertUtf16OffsetToUtf8(n), i);
            if (s === 0) return null;
            const a = o.HEAPU32;
            let l = s / 4;
            const c = a[l++], d = a[l++], _ = [];
            for(let p = 0; p < d; p++){
                const f = e.convertUtf8OffsetToUtf16(a[l++]), h = e.convertUtf8OffsetToUtf16(a[l++]);
                _[p] = {
                    start: f,
                    end: h,
                    length: h - f
                };
            }
            return {
                index: c,
                captureIndices: _
            };
        }
    }
    function $s(t) {
        return typeof t.instantiator == "function";
    }
    function js(t) {
        return typeof t.default == "function";
    }
    function Bs(t) {
        return typeof t.data < "u";
    }
    function Gs(t) {
        return typeof Response < "u" && t instanceof Response;
    }
    function Us(t) {
        return typeof ArrayBuffer < "u" && (t instanceof ArrayBuffer || ArrayBuffer.isView(t)) || typeof Buffer < "u" && Buffer.isBuffer?.(t) || typeof SharedArrayBuffer < "u" && t instanceof SharedArrayBuffer || typeof Uint32Array < "u" && t instanceof Uint32Array;
    }
    let rt;
    function Hs(t) {
        if (rt) return rt;
        async function e() {
            G = await xs(async (n)=>{
                let r = t;
                return r = await r, typeof r == "function" && (r = await r(n)), typeof r == "function" && (r = await r(n)), $s(r) ? r = await r.instantiator(n) : js(r) ? r = await r.default(n) : (Bs(r) && (r = r.data), Gs(r) ? typeof WebAssembly.instantiateStreaming == "function" ? r = await Fs(r)(n) : r = await Ws(r)(n) : Us(r) ? r = await $t(r)(n) : r instanceof WebAssembly.Module ? r = await $t(r)(n) : "default" in r && r.default instanceof WebAssembly.Module && (r = await $t(r.default)(n))), "instance" in r && (r = r.instance), "exports" in r && (r = r.exports), r;
            });
        }
        return rt = e(), rt;
    }
    function $t(t) {
        return (e)=>WebAssembly.instantiate(t, e);
    }
    function Fs(t) {
        return (e)=>WebAssembly.instantiateStreaming(t, e);
    }
    function Ws(t) {
        return async (e)=>{
            const n = await t.arrayBuffer();
            return WebAssembly.instantiate(n, e);
        };
    }
    let qs;
    function zs() {
        return qs;
    }
    async function fr(t) {
        return t && await Hs(t), {
            createScanner (e) {
                return new Ms(e.map((n)=>typeof n == "string" ? n : n.source));
            },
            createString (e) {
                return new Je(e);
            }
        };
    }
    function Ks(t) {
        return cn(t);
    }
    function cn(t) {
        return Array.isArray(t) ? Js(t) : t instanceof RegExp ? t : typeof t == "object" ? Xs(t) : t;
    }
    function Js(t) {
        let e = [];
        for(let n = 0, r = t.length; n < r; n++)e[n] = cn(t[n]);
        return e;
    }
    function Xs(t) {
        let e = {};
        for(let n in t)e[n] = cn(t[n]);
        return e;
    }
    function pr(t, ...e) {
        return e.forEach((n)=>{
            for(let r in n)t[r] = n[r];
        }), t;
    }
    function mr(t) {
        const e = ~t.lastIndexOf("/") || ~t.lastIndexOf("\\");
        return e === 0 ? t : ~e === t.length - 1 ? mr(t.substring(0, t.length - 1)) : t.substr(~e + 1);
    }
    var jt = /\$(\d+)|\${(\d+):\/(downcase|upcase)}/g, it = class {
        static hasCaptures(t) {
            return t === null ? !1 : (jt.lastIndex = 0, jt.test(t));
        }
        static replaceCaptures(t, e, n) {
            return t.replace(jt, (r, i, o, s)=>{
                let a = n[parseInt(i || o, 10)];
                if (a) {
                    let l = e.substring(a.start, a.end);
                    for(; l[0] === ".";)l = l.substring(1);
                    switch(s){
                        case "downcase":
                            return l.toLowerCase();
                        case "upcase":
                            return l.toUpperCase();
                        default:
                            return l;
                    }
                } else return r;
            });
        }
    };
    function gr(t, e) {
        return t < e ? -1 : t > e ? 1 : 0;
    }
    function br(t, e) {
        if (t === null && e === null) return 0;
        if (!t) return -1;
        if (!e) return 1;
        let n = t.length, r = e.length;
        if (n === r) {
            for(let i = 0; i < n; i++){
                let o = gr(t[i], e[i]);
                if (o !== 0) return o;
            }
            return 0;
        }
        return n - r;
    }
    function In(t) {
        return !!(/^#[0-9a-f]{6}$/i.test(t) || /^#[0-9a-f]{8}$/i.test(t) || /^#[0-9a-f]{3}$/i.test(t) || /^#[0-9a-f]{4}$/i.test(t));
    }
    function yr(t) {
        return t.replace(/[\-\\\{\}\*\+\?\|\^\$\.\,\[\]\(\)\#\s]/g, "\\$&");
    }
    var wr = class {
        constructor(t){
            this.fn = t;
        }
        cache = new Map;
        get(t) {
            if (this.cache.has(t)) return this.cache.get(t);
            const e = this.fn(t);
            return this.cache.set(t, e), e;
        }
    }, mt = class {
        constructor(t, e, n){
            this._colorMap = t, this._defaults = e, this._root = n;
        }
        static createFromRawTheme(t, e) {
            return this.createFromParsedTheme(Zs(t), e);
        }
        static createFromParsedTheme(t, e) {
            return ta(t, e);
        }
        _cachedMatchRoot = new wr((t)=>this._root.match(t));
        getColorMap() {
            return this._colorMap.getColorMap();
        }
        getDefaults() {
            return this._defaults;
        }
        match(t) {
            if (t === null) return this._defaults;
            const e = t.scopeName, r = this._cachedMatchRoot.get(e).find((i)=>Ys(t.parent, i.parentScopes));
            return r ? new Er(r.fontStyle, r.foreground, r.background) : null;
        }
    }, Bt = class ct {
        constructor(e, n){
            this.parent = e, this.scopeName = n;
        }
        static push(e, n) {
            for (const r of n)e = new ct(e, r);
            return e;
        }
        static from(...e) {
            let n = null;
            for(let r = 0; r < e.length; r++)n = new ct(n, e[r]);
            return n;
        }
        push(e) {
            return new ct(this, e);
        }
        getSegments() {
            let e = this;
            const n = [];
            for(; e;)n.push(e.scopeName), e = e.parent;
            return n.reverse(), n;
        }
        toString() {
            return this.getSegments().join(" ");
        }
        extends(e) {
            return this === e ? !0 : this.parent === null ? !1 : this.parent.extends(e);
        }
        getExtensionIfDefined(e) {
            const n = [];
            let r = this;
            for(; r && r !== e;)n.push(r.scopeName), r = r.parent;
            return r === e ? n.reverse() : void 0;
        }
    };
    function Ys(t, e) {
        if (e.length === 0) return !0;
        for(let n = 0; n < e.length; n++){
            let r = e[n], i = !1;
            if (r === ">") {
                if (n === e.length - 1) return !1;
                r = e[++n], i = !0;
            }
            for(; t && !Qs(t.scopeName, r);){
                if (i) return !1;
                t = t.parent;
            }
            if (!t) return !1;
            t = t.parent;
        }
        return !0;
    }
    function Qs(t, e) {
        return e === t || t.startsWith(e) && t[e.length] === ".";
    }
    var Er = class {
        constructor(t, e, n){
            this.fontStyle = t, this.foregroundId = e, this.backgroundId = n;
        }
    };
    function Zs(t) {
        if (!t) return [];
        if (!t.settings || !Array.isArray(t.settings)) return [];
        let e = t.settings, n = [], r = 0;
        for(let i = 0, o = e.length; i < o; i++){
            let s = e[i];
            if (!s.settings) continue;
            let a;
            if (typeof s.scope == "string") {
                let _ = s.scope;
                _ = _.replace(/^[,]+/, ""), _ = _.replace(/[,]+$/, ""), a = _.split(",");
            } else Array.isArray(s.scope) ? a = s.scope : a = [
                ""
            ];
            let l = -1;
            if (typeof s.settings.fontStyle == "string") {
                l = 0;
                let _ = s.settings.fontStyle.split(" ");
                for(let p = 0, f = _.length; p < f; p++)switch(_[p]){
                    case "italic":
                        l = l | 1;
                        break;
                    case "bold":
                        l = l | 2;
                        break;
                    case "underline":
                        l = l | 4;
                        break;
                    case "strikethrough":
                        l = l | 8;
                        break;
                }
            }
            let c = null;
            typeof s.settings.foreground == "string" && In(s.settings.foreground) && (c = s.settings.foreground);
            let d = null;
            typeof s.settings.background == "string" && In(s.settings.background) && (d = s.settings.background);
            for(let _ = 0, p = a.length; _ < p; _++){
                let h = a[_].trim().split(" "), E = h[h.length - 1], g = null;
                h.length > 1 && (g = h.slice(0, h.length - 1), g.reverse()), n[r++] = new ea(E, g, i, l, c, d);
            }
        }
        return n;
    }
    var ea = class {
        constructor(t, e, n, r, i, o){
            this.scope = t, this.parentScopes = e, this.index = n, this.fontStyle = r, this.foreground = i, this.background = o;
        }
    }, ae = ((t)=>(t[t.NotSet = -1] = "NotSet", t[t.None = 0] = "None", t[t.Italic = 1] = "Italic", t[t.Bold = 2] = "Bold", t[t.Underline = 4] = "Underline", t[t.Strikethrough = 8] = "Strikethrough", t))(ae || {});
    function ta(t, e) {
        t.sort((l, c)=>{
            let d = gr(l.scope, c.scope);
            return d !== 0 || (d = br(l.parentScopes, c.parentScopes), d !== 0) ? d : l.index - c.index;
        });
        let n = 0, r = "#000000", i = "#ffffff";
        for(; t.length >= 1 && t[0].scope === "";){
            let l = t.shift();
            l.fontStyle !== -1 && (n = l.fontStyle), l.foreground !== null && (r = l.foreground), l.background !== null && (i = l.background);
        }
        let o = new na(e), s = new Er(n, o.getId(r), o.getId(i)), a = new ia(new Jt(0, null, -1, 0, 0), []);
        for(let l = 0, c = t.length; l < c; l++){
            let d = t[l];
            a.insert(0, d.scope, d.parentScopes, d.fontStyle, o.getId(d.foreground), o.getId(d.background));
        }
        return new mt(o, s, a);
    }
    var na = class {
        _isFrozen;
        _lastColorId;
        _id2color;
        _color2id;
        constructor(t){
            if (this._lastColorId = 0, this._id2color = [], this._color2id = Object.create(null), Array.isArray(t)) {
                this._isFrozen = !0;
                for(let e = 0, n = t.length; e < n; e++)this._color2id[t[e]] = e, this._id2color[e] = t[e];
            } else this._isFrozen = !1;
        }
        getId(t) {
            if (t === null) return 0;
            t = t.toUpperCase();
            let e = this._color2id[t];
            if (e) return e;
            if (this._isFrozen) throw new Error(`Missing color in color map - ${t}`);
            return e = ++this._lastColorId, this._color2id[t] = e, this._id2color[e] = t, e;
        }
        getColorMap() {
            return this._id2color.slice(0);
        }
    }, ra = Object.freeze([]), Jt = class vr {
        scopeDepth;
        parentScopes;
        fontStyle;
        foreground;
        background;
        constructor(e, n, r, i, o){
            this.scopeDepth = e, this.parentScopes = n || ra, this.fontStyle = r, this.foreground = i, this.background = o;
        }
        clone() {
            return new vr(this.scopeDepth, this.parentScopes, this.fontStyle, this.foreground, this.background);
        }
        static cloneArr(e) {
            let n = [];
            for(let r = 0, i = e.length; r < i; r++)n[r] = e[r].clone();
            return n;
        }
        acceptOverwrite(e, n, r, i) {
            this.scopeDepth > e ? console.log("how did this happen?") : this.scopeDepth = e, n !== -1 && (this.fontStyle = n), r !== 0 && (this.foreground = r), i !== 0 && (this.background = i);
        }
    }, ia = class Xt {
        constructor(e, n = [], r = {}){
            this._mainRule = e, this._children = r, this._rulesWithParentScopes = n;
        }
        _rulesWithParentScopes;
        static _cmpBySpecificity(e, n) {
            if (e.scopeDepth !== n.scopeDepth) return n.scopeDepth - e.scopeDepth;
            let r = 0, i = 0;
            for(; e.parentScopes[r] === ">" && r++, n.parentScopes[i] === ">" && i++, !(r >= e.parentScopes.length || i >= n.parentScopes.length);){
                const o = n.parentScopes[i].length - e.parentScopes[r].length;
                if (o !== 0) return o;
                r++, i++;
            }
            return n.parentScopes.length - e.parentScopes.length;
        }
        match(e) {
            if (e !== "") {
                let r = e.indexOf("."), i, o;
                if (r === -1 ? (i = e, o = "") : (i = e.substring(0, r), o = e.substring(r + 1)), this._children.hasOwnProperty(i)) return this._children[i].match(o);
            }
            const n = this._rulesWithParentScopes.concat(this._mainRule);
            return n.sort(Xt._cmpBySpecificity), n;
        }
        insert(e, n, r, i, o, s) {
            if (n === "") {
                this._doInsertHere(e, r, i, o, s);
                return;
            }
            let a = n.indexOf("."), l, c;
            a === -1 ? (l = n, c = "") : (l = n.substring(0, a), c = n.substring(a + 1));
            let d;
            this._children.hasOwnProperty(l) ? d = this._children[l] : (d = new Xt(this._mainRule.clone(), Jt.cloneArr(this._rulesWithParentScopes)), this._children[l] = d), d.insert(e + 1, c, r, i, o, s);
        }
        _doInsertHere(e, n, r, i, o) {
            if (n === null) {
                this._mainRule.acceptOverwrite(e, r, i, o);
                return;
            }
            for(let s = 0, a = this._rulesWithParentScopes.length; s < a; s++){
                let l = this._rulesWithParentScopes[s];
                if (br(l.parentScopes, n) === 0) {
                    l.acceptOverwrite(e, r, i, o);
                    return;
                }
            }
            r === -1 && (r = this._mainRule.fontStyle), i === 0 && (i = this._mainRule.foreground), o === 0 && (o = this._mainRule.background), this._rulesWithParentScopes.push(new Jt(e, n, r, i, o));
        }
    }, ke = class Q {
        static toBinaryStr(e) {
            return e.toString(2).padStart(32, "0");
        }
        static print(e) {
            const n = Q.getLanguageId(e), r = Q.getTokenType(e), i = Q.getFontStyle(e), o = Q.getForeground(e), s = Q.getBackground(e);
            console.log({
                languageId: n,
                tokenType: r,
                fontStyle: i,
                foreground: o,
                background: s
            });
        }
        static getLanguageId(e) {
            return (e & 255) >>> 0;
        }
        static getTokenType(e) {
            return (e & 768) >>> 8;
        }
        static containsBalancedBrackets(e) {
            return (e & 1024) !== 0;
        }
        static getFontStyle(e) {
            return (e & 30720) >>> 11;
        }
        static getForeground(e) {
            return (e & 16744448) >>> 15;
        }
        static getBackground(e) {
            return (e & 4278190080) >>> 24;
        }
        static set(e, n, r, i, o, s, a) {
            let l = Q.getLanguageId(e), c = Q.getTokenType(e), d = Q.containsBalancedBrackets(e) ? 1 : 0, _ = Q.getFontStyle(e), p = Q.getForeground(e), f = Q.getBackground(e);
            return n !== 0 && (l = n), r !== 8 && (c = r), i !== null && (d = i ? 1 : 0), o !== -1 && (_ = o), s !== 0 && (p = s), a !== 0 && (f = a), (l << 0 | c << 8 | d << 10 | _ << 11 | p << 15 | f << 24) >>> 0;
        }
    };
    function gt(t, e) {
        const n = [], r = oa(t);
        let i = r.next();
        for(; i !== null;){
            let l = 0;
            if (i.length === 2 && i.charAt(1) === ":") {
                switch(i.charAt(0)){
                    case "R":
                        l = 1;
                        break;
                    case "L":
                        l = -1;
                        break;
                    default:
                        console.log(`Unknown priority ${i} in scope selector`);
                }
                i = r.next();
            }
            let c = s();
            if (n.push({
                matcher: c,
                priority: l
            }), i !== ",") break;
            i = r.next();
        }
        return n;
        function o() {
            if (i === "-") {
                i = r.next();
                const l = o();
                return (c)=>!!l && !l(c);
            }
            if (i === "(") {
                i = r.next();
                const l = a();
                return i === ")" && (i = r.next()), l;
            }
            if (On(i)) {
                const l = [];
                do l.push(i), i = r.next();
                while (On(i));
                return (c)=>e(l, c);
            }
            return null;
        }
        function s() {
            const l = [];
            let c = o();
            for(; c;)l.push(c), c = o();
            return (d)=>l.every((_)=>_(d));
        }
        function a() {
            const l = [];
            let c = s();
            for(; c && (l.push(c), i === "|" || i === ",");){
                do i = r.next();
                while (i === "|" || i === ",");
                c = s();
            }
            return (d)=>l.some((_)=>_(d));
        }
    }
    function On(t) {
        return !!t && !!t.match(/[\w\.:]+/);
    }
    function oa(t) {
        let e = /([LR]:|[\w\.:][\w\.:\-]*|[\,\|\-\(\)])/g, n = e.exec(t);
        return {
            next: ()=>{
                if (!n) return null;
                const r = n[0];
                return n = e.exec(t), r;
            }
        };
    }
    function Sr(t) {
        typeof t.dispose == "function" && t.dispose();
    }
    var He = class {
        constructor(t){
            this.scopeName = t;
        }
        toKey() {
            return this.scopeName;
        }
    }, sa = class {
        constructor(t, e){
            this.scopeName = t, this.ruleName = e;
        }
        toKey() {
            return `${this.scopeName}#${this.ruleName}`;
        }
    }, aa = class {
        _references = [];
        _seenReferenceKeys = new Set;
        get references() {
            return this._references;
        }
        visitedRule = new Set;
        add(t) {
            const e = t.toKey();
            this._seenReferenceKeys.has(e) || (this._seenReferenceKeys.add(e), this._references.push(t));
        }
    }, la = class {
        constructor(t, e){
            this.repo = t, this.initialScopeName = e, this.seenFullScopeRequests.add(this.initialScopeName), this.Q = [
                new He(this.initialScopeName)
            ];
        }
        seenFullScopeRequests = new Set;
        seenPartialScopeRequests = new Set;
        Q;
        processQueue() {
            const t = this.Q;
            this.Q = [];
            const e = new aa;
            for (const n of t)ca(n, this.initialScopeName, this.repo, e);
            for (const n of e.references)if (n instanceof He) {
                if (this.seenFullScopeRequests.has(n.scopeName)) continue;
                this.seenFullScopeRequests.add(n.scopeName), this.Q.push(n);
            } else {
                if (this.seenFullScopeRequests.has(n.scopeName) || this.seenPartialScopeRequests.has(n.toKey())) continue;
                this.seenPartialScopeRequests.add(n.toKey()), this.Q.push(n);
            }
        }
    };
    function ca(t, e, n, r) {
        const i = n.lookup(t.scopeName);
        if (!i) {
            if (t.scopeName === e) throw new Error(`No grammar provided for <${e}>`);
            return;
        }
        const o = n.lookup(e);
        t instanceof He ? ut({
            baseGrammar: o,
            selfGrammar: i
        }, r) : Yt(t.ruleName, {
            baseGrammar: o,
            selfGrammar: i,
            repository: i.repository
        }, r);
        const s = n.injections(t.scopeName);
        if (s) for (const a of s)r.add(new He(a));
    }
    function Yt(t, e, n) {
        if (e.repository && e.repository[t]) {
            const r = e.repository[t];
            bt([
                r
            ], e, n);
        }
    }
    function ut(t, e) {
        t.selfGrammar.patterns && Array.isArray(t.selfGrammar.patterns) && bt(t.selfGrammar.patterns, {
            ...t,
            repository: t.selfGrammar.repository
        }, e), t.selfGrammar.injections && bt(Object.values(t.selfGrammar.injections), {
            ...t,
            repository: t.selfGrammar.repository
        }, e);
    }
    function bt(t, e, n) {
        for (const r of t){
            if (n.visitedRule.has(r)) continue;
            n.visitedRule.add(r);
            const i = r.repository ? pr({}, e.repository, r.repository) : e.repository;
            Array.isArray(r.patterns) && bt(r.patterns, {
                ...e,
                repository: i
            }, n);
            const o = r.include;
            if (!o) continue;
            const s = Ar(o);
            switch(s.kind){
                case 0:
                    ut({
                        ...e,
                        selfGrammar: e.baseGrammar
                    }, n);
                    break;
                case 1:
                    ut(e, n);
                    break;
                case 2:
                    Yt(s.ruleName, {
                        ...e,
                        repository: i
                    }, n);
                    break;
                case 3:
                case 4:
                    const a = s.scopeName === e.selfGrammar.scopeName ? e.selfGrammar : s.scopeName === e.baseGrammar.scopeName ? e.baseGrammar : void 0;
                    if (a) {
                        const l = {
                            baseGrammar: e.baseGrammar,
                            selfGrammar: a,
                            repository: i
                        };
                        s.kind === 4 ? Yt(s.ruleName, l, n) : ut(l, n);
                    } else s.kind === 4 ? n.add(new sa(s.scopeName, s.ruleName)) : n.add(new He(s.scopeName));
                    break;
            }
        }
    }
    var ua = class {
        kind = 0;
    }, da = class {
        kind = 1;
    }, _a = class {
        constructor(t){
            this.ruleName = t;
        }
        kind = 2;
    }, ha = class {
        constructor(t){
            this.scopeName = t;
        }
        kind = 3;
    }, fa = class {
        constructor(t, e){
            this.scopeName = t, this.ruleName = e;
        }
        kind = 4;
    };
    function Ar(t) {
        if (t === "$base") return new ua;
        if (t === "$self") return new da;
        const e = t.indexOf("#");
        if (e === -1) return new ha(t);
        if (e === 0) return new _a(t.substring(1));
        {
            const n = t.substring(0, e), r = t.substring(e + 1);
            return new fa(n, r);
        }
    }
    var pa = /\\(\d+)/, xn = /\\(\d+)/g, ma = -1, Tr = -2;
    var Xe = class {
        $location;
        id;
        _nameIsCapturing;
        _name;
        _contentNameIsCapturing;
        _contentName;
        constructor(t, e, n, r){
            this.$location = t, this.id = e, this._name = n || null, this._nameIsCapturing = it.hasCaptures(this._name), this._contentName = r || null, this._contentNameIsCapturing = it.hasCaptures(this._contentName);
        }
        get debugName() {
            const t = this.$location ? `${mr(this.$location.filename)}:${this.$location.line}` : "unknown";
            return `${this.constructor.name}#${this.id} @ ${t}`;
        }
        getName(t, e) {
            return !this._nameIsCapturing || this._name === null || t === null || e === null ? this._name : it.replaceCaptures(this._name, t, e);
        }
        getContentName(t, e) {
            return !this._contentNameIsCapturing || this._contentName === null ? this._contentName : it.replaceCaptures(this._contentName, t, e);
        }
    }, ga = class extends Xe {
        retokenizeCapturedWithRuleId;
        constructor(t, e, n, r, i){
            super(t, e, n, r), this.retokenizeCapturedWithRuleId = i;
        }
        dispose() {}
        collectPatterns(t, e) {
            throw new Error("Not supported!");
        }
        compile(t, e) {
            throw new Error("Not supported!");
        }
        compileAG(t, e, n, r) {
            throw new Error("Not supported!");
        }
    }, ba = class extends Xe {
        _match;
        captures;
        _cachedCompiledPatterns;
        constructor(t, e, n, r, i){
            super(t, e, n, null), this._match = new Fe(r, this.id), this.captures = i, this._cachedCompiledPatterns = null;
        }
        dispose() {
            this._cachedCompiledPatterns && (this._cachedCompiledPatterns.dispose(), this._cachedCompiledPatterns = null);
        }
        get debugMatchRegExp() {
            return `${this._match.source}`;
        }
        collectPatterns(t, e) {
            e.push(this._match);
        }
        compile(t, e) {
            return this._getCachedCompiledPatterns(t).compile(t);
        }
        compileAG(t, e, n, r) {
            return this._getCachedCompiledPatterns(t).compileAG(t, n, r);
        }
        _getCachedCompiledPatterns(t) {
            return this._cachedCompiledPatterns || (this._cachedCompiledPatterns = new We, this.collectPatterns(t, this._cachedCompiledPatterns)), this._cachedCompiledPatterns;
        }
    }, Dn = class extends Xe {
        hasMissingPatterns;
        patterns;
        _cachedCompiledPatterns;
        constructor(t, e, n, r, i){
            super(t, e, n, r), this.patterns = i.patterns, this.hasMissingPatterns = i.hasMissingPatterns, this._cachedCompiledPatterns = null;
        }
        dispose() {
            this._cachedCompiledPatterns && (this._cachedCompiledPatterns.dispose(), this._cachedCompiledPatterns = null);
        }
        collectPatterns(t, e) {
            for (const n of this.patterns)t.getRule(n).collectPatterns(t, e);
        }
        compile(t, e) {
            return this._getCachedCompiledPatterns(t).compile(t);
        }
        compileAG(t, e, n, r) {
            return this._getCachedCompiledPatterns(t).compileAG(t, n, r);
        }
        _getCachedCompiledPatterns(t) {
            return this._cachedCompiledPatterns || (this._cachedCompiledPatterns = new We, this.collectPatterns(t, this._cachedCompiledPatterns)), this._cachedCompiledPatterns;
        }
    }, Qt = class extends Xe {
        _begin;
        beginCaptures;
        _end;
        endHasBackReferences;
        endCaptures;
        applyEndPatternLast;
        hasMissingPatterns;
        patterns;
        _cachedCompiledPatterns;
        constructor(t, e, n, r, i, o, s, a, l, c){
            super(t, e, n, r), this._begin = new Fe(i, this.id), this.beginCaptures = o, this._end = new Fe(s || "￿", -1), this.endHasBackReferences = this._end.hasBackReferences, this.endCaptures = a, this.applyEndPatternLast = l || !1, this.patterns = c.patterns, this.hasMissingPatterns = c.hasMissingPatterns, this._cachedCompiledPatterns = null;
        }
        dispose() {
            this._cachedCompiledPatterns && (this._cachedCompiledPatterns.dispose(), this._cachedCompiledPatterns = null);
        }
        get debugBeginRegExp() {
            return `${this._begin.source}`;
        }
        get debugEndRegExp() {
            return `${this._end.source}`;
        }
        getEndWithResolvedBackReferences(t, e) {
            return this._end.resolveBackReferences(t, e);
        }
        collectPatterns(t, e) {
            e.push(this._begin);
        }
        compile(t, e) {
            return this._getCachedCompiledPatterns(t, e).compile(t);
        }
        compileAG(t, e, n, r) {
            return this._getCachedCompiledPatterns(t, e).compileAG(t, n, r);
        }
        _getCachedCompiledPatterns(t, e) {
            if (!this._cachedCompiledPatterns) {
                this._cachedCompiledPatterns = new We;
                for (const n of this.patterns)t.getRule(n).collectPatterns(t, this._cachedCompiledPatterns);
                this.applyEndPatternLast ? this._cachedCompiledPatterns.push(this._end.hasBackReferences ? this._end.clone() : this._end) : this._cachedCompiledPatterns.unshift(this._end.hasBackReferences ? this._end.clone() : this._end);
            }
            return this._end.hasBackReferences && (this.applyEndPatternLast ? this._cachedCompiledPatterns.setSource(this._cachedCompiledPatterns.length() - 1, e) : this._cachedCompiledPatterns.setSource(0, e)), this._cachedCompiledPatterns;
        }
    }, yt = class extends Xe {
        _begin;
        beginCaptures;
        whileCaptures;
        _while;
        whileHasBackReferences;
        hasMissingPatterns;
        patterns;
        _cachedCompiledPatterns;
        _cachedCompiledWhilePatterns;
        constructor(t, e, n, r, i, o, s, a, l){
            super(t, e, n, r), this._begin = new Fe(i, this.id), this.beginCaptures = o, this.whileCaptures = a, this._while = new Fe(s, Tr), this.whileHasBackReferences = this._while.hasBackReferences, this.patterns = l.patterns, this.hasMissingPatterns = l.hasMissingPatterns, this._cachedCompiledPatterns = null, this._cachedCompiledWhilePatterns = null;
        }
        dispose() {
            this._cachedCompiledPatterns && (this._cachedCompiledPatterns.dispose(), this._cachedCompiledPatterns = null), this._cachedCompiledWhilePatterns && (this._cachedCompiledWhilePatterns.dispose(), this._cachedCompiledWhilePatterns = null);
        }
        get debugBeginRegExp() {
            return `${this._begin.source}`;
        }
        get debugWhileRegExp() {
            return `${this._while.source}`;
        }
        getWhileWithResolvedBackReferences(t, e) {
            return this._while.resolveBackReferences(t, e);
        }
        collectPatterns(t, e) {
            e.push(this._begin);
        }
        compile(t, e) {
            return this._getCachedCompiledPatterns(t).compile(t);
        }
        compileAG(t, e, n, r) {
            return this._getCachedCompiledPatterns(t).compileAG(t, n, r);
        }
        _getCachedCompiledPatterns(t) {
            if (!this._cachedCompiledPatterns) {
                this._cachedCompiledPatterns = new We;
                for (const e of this.patterns)t.getRule(e).collectPatterns(t, this._cachedCompiledPatterns);
            }
            return this._cachedCompiledPatterns;
        }
        compileWhile(t, e) {
            return this._getCachedCompiledWhilePatterns(t, e).compile(t);
        }
        compileWhileAG(t, e, n, r) {
            return this._getCachedCompiledWhilePatterns(t, e).compileAG(t, n, r);
        }
        _getCachedCompiledWhilePatterns(t, e) {
            return this._cachedCompiledWhilePatterns || (this._cachedCompiledWhilePatterns = new We, this._cachedCompiledWhilePatterns.push(this._while.hasBackReferences ? this._while.clone() : this._while)), this._while.hasBackReferences && this._cachedCompiledWhilePatterns.setSource(0, e || "￿"), this._cachedCompiledWhilePatterns;
        }
    }, Rr = class H {
        static createCaptureRule(e, n, r, i, o) {
            return e.registerRule((s)=>new ga(n, s, r, i, o));
        }
        static getCompiledRuleId(e, n, r) {
            return e.id || n.registerRule((i)=>{
                if (e.id = i, e.match) return new ba(e.$vscodeTextmateLocation, e.id, e.name, e.match, H._compileCaptures(e.captures, n, r));
                if (typeof e.begin > "u") {
                    e.repository && (r = pr({}, r, e.repository));
                    let o = e.patterns;
                    return typeof o > "u" && e.include && (o = [
                        {
                            include: e.include
                        }
                    ]), new Dn(e.$vscodeTextmateLocation, e.id, e.name, e.contentName, H._compilePatterns(o, n, r));
                }
                return e.while ? new yt(e.$vscodeTextmateLocation, e.id, e.name, e.contentName, e.begin, H._compileCaptures(e.beginCaptures || e.captures, n, r), e.while, H._compileCaptures(e.whileCaptures || e.captures, n, r), H._compilePatterns(e.patterns, n, r)) : new Qt(e.$vscodeTextmateLocation, e.id, e.name, e.contentName, e.begin, H._compileCaptures(e.beginCaptures || e.captures, n, r), e.end, H._compileCaptures(e.endCaptures || e.captures, n, r), e.applyEndPatternLast, H._compilePatterns(e.patterns, n, r));
            }), e.id;
        }
        static _compileCaptures(e, n, r) {
            let i = [];
            if (e) {
                let o = 0;
                for(const s in e){
                    if (s === "$vscodeTextmateLocation") continue;
                    const a = parseInt(s, 10);
                    a > o && (o = a);
                }
                for(let s = 0; s <= o; s++)i[s] = null;
                for(const s in e){
                    if (s === "$vscodeTextmateLocation") continue;
                    const a = parseInt(s, 10);
                    let l = 0;
                    e[s].patterns && (l = H.getCompiledRuleId(e[s], n, r)), i[a] = H.createCaptureRule(n, e[s].$vscodeTextmateLocation, e[s].name, e[s].contentName, l);
                }
            }
            return i;
        }
        static _compilePatterns(e, n, r) {
            let i = [];
            if (e) for(let o = 0, s = e.length; o < s; o++){
                const a = e[o];
                let l = -1;
                if (a.include) {
                    const c = Ar(a.include);
                    switch(c.kind){
                        case 0:
                        case 1:
                            l = H.getCompiledRuleId(r[a.include], n, r);
                            break;
                        case 2:
                            let d = r[c.ruleName];
                            d && (l = H.getCompiledRuleId(d, n, r));
                            break;
                        case 3:
                        case 4:
                            const _ = c.scopeName, p = c.kind === 4 ? c.ruleName : null, f = n.getExternalGrammar(_, r);
                            if (f) if (p) {
                                let h = f.repository[p];
                                h && (l = H.getCompiledRuleId(h, n, f.repository));
                            } else l = H.getCompiledRuleId(f.repository.$self, n, f.repository);
                            break;
                    }
                } else l = H.getCompiledRuleId(a, n, r);
                if (l !== -1) {
                    const c = n.getRule(l);
                    let d = !1;
                    if ((c instanceof Dn || c instanceof Qt || c instanceof yt) && c.hasMissingPatterns && c.patterns.length === 0 && (d = !0), d) continue;
                    i.push(l);
                }
            }
            return {
                patterns: i,
                hasMissingPatterns: (e ? e.length : 0) !== i.length
            };
        }
    }, Fe = class Lr {
        source;
        ruleId;
        hasAnchor;
        hasBackReferences;
        _anchorCache;
        constructor(e, n){
            if (e && typeof e == "string") {
                const r = e.length;
                let i = 0, o = [], s = !1;
                for(let a = 0; a < r; a++)if (e.charAt(a) === "\\" && a + 1 < r) {
                    const c = e.charAt(a + 1);
                    c === "z" ? (o.push(e.substring(i, a)), o.push("$(?!\\n)(?<!\\n)"), i = a + 2) : (c === "A" || c === "G") && (s = !0), a++;
                }
                this.hasAnchor = s, i === 0 ? this.source = e : (o.push(e.substring(i, r)), this.source = o.join(""));
            } else this.hasAnchor = !1, this.source = e;
            this.hasAnchor ? this._anchorCache = this._buildAnchorCache() : this._anchorCache = null, this.ruleId = n, typeof this.source == "string" ? this.hasBackReferences = pa.test(this.source) : this.hasBackReferences = !1;
        }
        clone() {
            return new Lr(this.source, this.ruleId);
        }
        setSource(e) {
            this.source !== e && (this.source = e, this.hasAnchor && (this._anchorCache = this._buildAnchorCache()));
        }
        resolveBackReferences(e, n) {
            if (typeof this.source != "string") throw new Error("This method should only be called if the source is a string");
            let r = n.map((i)=>e.substring(i.start, i.end));
            return xn.lastIndex = 0, this.source.replace(xn, (i, o)=>yr(r[parseInt(o, 10)] || ""));
        }
        _buildAnchorCache() {
            if (typeof this.source != "string") throw new Error("This method should only be called if the source is a string");
            let e = [], n = [], r = [], i = [], o, s, a, l;
            for(o = 0, s = this.source.length; o < s; o++)a = this.source.charAt(o), e[o] = a, n[o] = a, r[o] = a, i[o] = a, a === "\\" && o + 1 < s && (l = this.source.charAt(o + 1), l === "A" ? (e[o + 1] = "￿", n[o + 1] = "￿", r[o + 1] = "A", i[o + 1] = "A") : l === "G" ? (e[o + 1] = "￿", n[o + 1] = "G", r[o + 1] = "￿", i[o + 1] = "G") : (e[o + 1] = l, n[o + 1] = l, r[o + 1] = l, i[o + 1] = l), o++);
            return {
                A0_G0: e.join(""),
                A0_G1: n.join(""),
                A1_G0: r.join(""),
                A1_G1: i.join("")
            };
        }
        resolveAnchors(e, n) {
            return !this.hasAnchor || !this._anchorCache || typeof this.source != "string" ? this.source : e ? n ? this._anchorCache.A1_G1 : this._anchorCache.A1_G0 : n ? this._anchorCache.A0_G1 : this._anchorCache.A0_G0;
        }
    }, We = class {
        _items;
        _hasAnchors;
        _cached;
        _anchorCache;
        constructor(){
            this._items = [], this._hasAnchors = !1, this._cached = null, this._anchorCache = {
                A0_G0: null,
                A0_G1: null,
                A1_G0: null,
                A1_G1: null
            };
        }
        dispose() {
            this._disposeCaches();
        }
        _disposeCaches() {
            this._cached && (this._cached.dispose(), this._cached = null), this._anchorCache.A0_G0 && (this._anchorCache.A0_G0.dispose(), this._anchorCache.A0_G0 = null), this._anchorCache.A0_G1 && (this._anchorCache.A0_G1.dispose(), this._anchorCache.A0_G1 = null), this._anchorCache.A1_G0 && (this._anchorCache.A1_G0.dispose(), this._anchorCache.A1_G0 = null), this._anchorCache.A1_G1 && (this._anchorCache.A1_G1.dispose(), this._anchorCache.A1_G1 = null);
        }
        push(t) {
            this._items.push(t), this._hasAnchors = this._hasAnchors || t.hasAnchor;
        }
        unshift(t) {
            this._items.unshift(t), this._hasAnchors = this._hasAnchors || t.hasAnchor;
        }
        length() {
            return this._items.length;
        }
        setSource(t, e) {
            this._items[t].source !== e && (this._disposeCaches(), this._items[t].setSource(e));
        }
        compile(t) {
            if (!this._cached) {
                let e = this._items.map((n)=>n.source);
                this._cached = new Nn(t, e, this._items.map((n)=>n.ruleId));
            }
            return this._cached;
        }
        compileAG(t, e, n) {
            return this._hasAnchors ? e ? n ? (this._anchorCache.A1_G1 || (this._anchorCache.A1_G1 = this._resolveAnchors(t, e, n)), this._anchorCache.A1_G1) : (this._anchorCache.A1_G0 || (this._anchorCache.A1_G0 = this._resolveAnchors(t, e, n)), this._anchorCache.A1_G0) : n ? (this._anchorCache.A0_G1 || (this._anchorCache.A0_G1 = this._resolveAnchors(t, e, n)), this._anchorCache.A0_G1) : (this._anchorCache.A0_G0 || (this._anchorCache.A0_G0 = this._resolveAnchors(t, e, n)), this._anchorCache.A0_G0) : this.compile(t);
        }
        _resolveAnchors(t, e, n) {
            let r = this._items.map((i)=>i.resolveAnchors(e, n));
            return new Nn(t, r, this._items.map((i)=>i.ruleId));
        }
    }, Nn = class {
        constructor(t, e, n){
            this.regExps = e, this.rules = n, this.scanner = t.createOnigScanner(e);
        }
        scanner;
        dispose() {
            typeof this.scanner.dispose == "function" && this.scanner.dispose();
        }
        toString() {
            const t = [];
            for(let e = 0, n = this.rules.length; e < n; e++)t.push("   - " + this.rules[e] + ": " + this.regExps[e]);
            return t.join(`
`);
        }
        findNextMatchSync(t, e, n) {
            const r = this.scanner.findNextMatchSync(t, e, n);
            return r ? {
                ruleId: this.rules[r.index],
                captureIndices: r.captureIndices
            } : null;
        }
    }, Gt = class {
        constructor(t, e){
            this.languageId = t, this.tokenType = e;
        }
    }, ya = class Zt {
        _defaultAttributes;
        _embeddedLanguagesMatcher;
        constructor(e, n){
            this._defaultAttributes = new Gt(e, 8), this._embeddedLanguagesMatcher = new wa(Object.entries(n || {}));
        }
        getDefaultAttributes() {
            return this._defaultAttributes;
        }
        getBasicScopeAttributes(e) {
            return e === null ? Zt._NULL_SCOPE_METADATA : this._getBasicScopeAttributes.get(e);
        }
        static _NULL_SCOPE_METADATA = new Gt(0, 0);
        _getBasicScopeAttributes = new wr((e)=>{
            const n = this._scopeToLanguage(e), r = this._toStandardTokenType(e);
            return new Gt(n, r);
        });
        _scopeToLanguage(e) {
            return this._embeddedLanguagesMatcher.match(e) || 0;
        }
        _toStandardTokenType(e) {
            const n = e.match(Zt.STANDARD_TOKEN_TYPE_REGEXP);
            if (!n) return 8;
            switch(n[1]){
                case "comment":
                    return 1;
                case "string":
                    return 2;
                case "regex":
                    return 3;
                case "meta.embedded":
                    return 0;
            }
            throw new Error("Unexpected match for standard token type!");
        }
        static STANDARD_TOKEN_TYPE_REGEXP = /\b(comment|string|regex|meta\.embedded)\b/;
    }, wa = class {
        values;
        scopesRegExp;
        constructor(t){
            if (t.length === 0) this.values = null, this.scopesRegExp = null;
            else {
                this.values = new Map(t);
                const e = t.map(([n, r])=>yr(n));
                e.sort(), e.reverse(), this.scopesRegExp = new RegExp(`^((${e.join(")|(")}))($|\\.)`, "");
            }
        }
        match(t) {
            if (!this.scopesRegExp) return;
            const e = t.match(this.scopesRegExp);
            if (e) return this.values.get(e[1]);
        }
    }, Vn = class {
        constructor(t, e){
            this.stack = t, this.stoppedEarly = e;
        }
    };
    function Pr(t, e, n, r, i, o, s, a) {
        const l = e.content.length;
        let c = !1, d = -1;
        if (s) {
            const f = Ea(t, e, n, r, i, o);
            i = f.stack, r = f.linePos, n = f.isFirstLine, d = f.anchorPosition;
        }
        const _ = Date.now();
        for(; !c;){
            if (a !== 0 && Date.now() - _ > a) return new Vn(i, !0);
            p();
        }
        return new Vn(i, !1);
        function p() {
            const f = va(t, e, n, r, i, d);
            if (!f) {
                o.produce(i, l), c = !0;
                return;
            }
            const h = f.captureIndices, E = f.matchedRuleId, g = h && h.length > 0 ? h[0].end > r : !1;
            if (E === ma) {
                const w = i.getRule(t);
                o.produce(i, h[0].start), i = i.withContentNameScopesList(i.nameScopesList), Ve(t, e, n, i, o, w.endCaptures, h), o.produce(i, h[0].end);
                const m = i;
                if (i = i.parent, d = m.getAnchorPos(), !g && m.getEnterPos() === r) {
                    i = m, o.produce(i, l), c = !0;
                    return;
                }
            } else {
                const w = t.getRule(E);
                o.produce(i, h[0].start);
                const m = i, y = w.getName(e.content, h), v = i.contentNameScopesList.pushAttributed(y, t);
                if (i = i.push(E, r, d, h[0].end === l, null, v, v), w instanceof Qt) {
                    const R = w;
                    Ve(t, e, n, i, o, R.beginCaptures, h), o.produce(i, h[0].end), d = h[0].end;
                    const O = R.getContentName(e.content, h), V = v.pushAttributed(O, t);
                    if (i = i.withContentNameScopesList(V), R.endHasBackReferences && (i = i.withEndRule(R.getEndWithResolvedBackReferences(e.content, h))), !g && m.hasSameRuleAs(i)) {
                        i = i.pop(), o.produce(i, l), c = !0;
                        return;
                    }
                } else if (w instanceof yt) {
                    const R = w;
                    Ve(t, e, n, i, o, R.beginCaptures, h), o.produce(i, h[0].end), d = h[0].end;
                    const O = R.getContentName(e.content, h), V = v.pushAttributed(O, t);
                    if (i = i.withContentNameScopesList(V), R.whileHasBackReferences && (i = i.withEndRule(R.getWhileWithResolvedBackReferences(e.content, h))), !g && m.hasSameRuleAs(i)) {
                        i = i.pop(), o.produce(i, l), c = !0;
                        return;
                    }
                } else if (Ve(t, e, n, i, o, w.captures, h), o.produce(i, h[0].end), i = i.pop(), !g) {
                    i = i.safePop(), o.produce(i, l), c = !0;
                    return;
                }
            }
            h[0].end > r && (r = h[0].end, n = !1);
        }
    }
    function Ea(t, e, n, r, i, o) {
        let s = i.beginRuleCapturedEOL ? 0 : -1;
        const a = [];
        for(let l = i; l; l = l.pop()){
            const c = l.getRule(t);
            c instanceof yt && a.push({
                rule: c,
                stack: l
            });
        }
        for(let l = a.pop(); l; l = a.pop()){
            const { ruleScanner: c, findOptions: d } = Ta(l.rule, t, l.stack.endRule, n, r === s), _ = c.findNextMatchSync(e, r, d);
            if (_) {
                if (_.ruleId !== Tr) {
                    i = l.stack.pop();
                    break;
                }
                _.captureIndices && _.captureIndices.length && (o.produce(l.stack, _.captureIndices[0].start), Ve(t, e, n, l.stack, o, l.rule.whileCaptures, _.captureIndices), o.produce(l.stack, _.captureIndices[0].end), s = _.captureIndices[0].end, _.captureIndices[0].end > r && (r = _.captureIndices[0].end, n = !1));
            } else {
                i = l.stack.pop();
                break;
            }
        }
        return {
            stack: i,
            linePos: r,
            anchorPosition: s,
            isFirstLine: n
        };
    }
    function va(t, e, n, r, i, o) {
        const s = Sa(t, e, n, r, i, o), a = t.getInjections();
        if (a.length === 0) return s;
        const l = Aa(a, t, e, n, r, i, o);
        if (!l) return s;
        if (!s) return l;
        const c = s.captureIndices[0].start, d = l.captureIndices[0].start;
        return d < c || l.priorityMatch && d === c ? l : s;
    }
    function Sa(t, e, n, r, i, o) {
        const s = i.getRule(t), { ruleScanner: a, findOptions: l } = kr(s, t, i.endRule, n, r === o), c = a.findNextMatchSync(e, r, l);
        return c ? {
            captureIndices: c.captureIndices,
            matchedRuleId: c.ruleId
        } : null;
    }
    function Aa(t, e, n, r, i, o, s) {
        let a = Number.MAX_VALUE, l = null, c, d = 0;
        const _ = o.contentNameScopesList.getScopeNames();
        for(let p = 0, f = t.length; p < f; p++){
            const h = t[p];
            if (!h.matcher(_)) continue;
            const E = e.getRule(h.ruleId), { ruleScanner: g, findOptions: w } = kr(E, e, null, r, i === s), m = g.findNextMatchSync(n, i, w);
            if (!m) continue;
            const y = m.captureIndices[0].start;
            if (!(y >= a) && (a = y, l = m.captureIndices, c = m.ruleId, d = h.priority, a === i)) break;
        }
        return l ? {
            priorityMatch: d === -1,
            captureIndices: l,
            matchedRuleId: c
        } : null;
    }
    function kr(t, e, n, r, i) {
        return {
            ruleScanner: t.compileAG(e, n, r, i),
            findOptions: 0
        };
    }
    function Ta(t, e, n, r, i) {
        return {
            ruleScanner: t.compileWhileAG(e, n, r, i),
            findOptions: 0
        };
    }
    function Ve(t, e, n, r, i, o, s) {
        if (o.length === 0) return;
        const a = e.content, l = Math.min(o.length, s.length), c = [], d = s[0].end;
        for(let _ = 0; _ < l; _++){
            const p = o[_];
            if (p === null) continue;
            const f = s[_];
            if (f.length === 0) continue;
            if (f.start > d) break;
            for(; c.length > 0 && c[c.length - 1].endPos <= f.start;)i.produceFromScopes(c[c.length - 1].scopes, c[c.length - 1].endPos), c.pop();
            if (c.length > 0 ? i.produceFromScopes(c[c.length - 1].scopes, f.start) : i.produce(r, f.start), p.retokenizeCapturedWithRuleId) {
                const E = p.getName(a, s), g = r.contentNameScopesList.pushAttributed(E, t), w = p.getContentName(a, s), m = g.pushAttributed(w, t), y = r.push(p.retokenizeCapturedWithRuleId, f.start, -1, !1, null, g, m), v = t.createOnigString(a.substring(0, f.end));
                Pr(t, v, n && f.start === 0, f.start, y, i, !1, 0), Sr(v);
                continue;
            }
            const h = p.getName(a, s);
            if (h !== null) {
                const g = (c.length > 0 ? c[c.length - 1].scopes : r.contentNameScopesList).pushAttributed(h, t);
                c.push(new Ra(g, f.end));
            }
        }
        for(; c.length > 0;)i.produceFromScopes(c[c.length - 1].scopes, c[c.length - 1].endPos), c.pop();
    }
    var Ra = class {
        scopes;
        endPos;
        constructor(t, e){
            this.scopes = t, this.endPos = e;
        }
    };
    function La(t, e, n, r, i, o, s, a) {
        return new ka(t, e, n, r, i, o, s, a);
    }
    function Mn(t, e, n, r, i) {
        const o = gt(e, wt), s = Rr.getCompiledRuleId(n, r, i.repository);
        for (const a of o)t.push({
            debugSelector: e,
            matcher: a.matcher,
            ruleId: s,
            grammar: i,
            priority: a.priority
        });
    }
    function wt(t, e) {
        if (e.length < t.length) return !1;
        let n = 0;
        return t.every((r)=>{
            for(let i = n; i < e.length; i++)if (Pa(e[i], r)) return n = i + 1, !0;
            return !1;
        });
    }
    function Pa(t, e) {
        if (!t) return !1;
        if (t === e) return !0;
        const n = e.length;
        return t.length > n && t.substr(0, n) === e && t[n] === ".";
    }
    var ka = class {
        constructor(t, e, n, r, i, o, s, a){
            if (this._rootScopeName = t, this.balancedBracketSelectors = o, this._onigLib = a, this._basicScopeAttributesProvider = new ya(n, r), this._rootId = -1, this._lastRuleId = 0, this._ruleId2desc = [
                null
            ], this._includedGrammars = {}, this._grammarRepository = s, this._grammar = $n(e, null), this._injections = null, this._tokenTypeMatchers = [], i) for (const l of Object.keys(i)){
                const c = gt(l, wt);
                for (const d of c)this._tokenTypeMatchers.push({
                    matcher: d.matcher,
                    type: i[l]
                });
            }
        }
        _rootId;
        _lastRuleId;
        _ruleId2desc;
        _includedGrammars;
        _grammarRepository;
        _grammar;
        _injections;
        _basicScopeAttributesProvider;
        _tokenTypeMatchers;
        get themeProvider() {
            return this._grammarRepository;
        }
        dispose() {
            for (const t of this._ruleId2desc)t && t.dispose();
        }
        createOnigScanner(t) {
            return this._onigLib.createOnigScanner(t);
        }
        createOnigString(t) {
            return this._onigLib.createOnigString(t);
        }
        getMetadataForScope(t) {
            return this._basicScopeAttributesProvider.getBasicScopeAttributes(t);
        }
        _collectInjections() {
            const t = {
                lookup: (i)=>i === this._rootScopeName ? this._grammar : this.getExternalGrammar(i),
                injections: (i)=>this._grammarRepository.injections(i)
            }, e = [], n = this._rootScopeName, r = t.lookup(n);
            if (r) {
                const i = r.injections;
                if (i) for(let s in i)Mn(e, s, i[s], this, r);
                const o = this._grammarRepository.injections(n);
                o && o.forEach((s)=>{
                    const a = this.getExternalGrammar(s);
                    if (a) {
                        const l = a.injectionSelector;
                        l && Mn(e, l, a, this, a);
                    }
                });
            }
            return e.sort((i, o)=>i.priority - o.priority), e;
        }
        getInjections() {
            return this._injections === null && (this._injections = this._collectInjections()), this._injections;
        }
        registerRule(t) {
            const e = ++this._lastRuleId, n = t(e);
            return this._ruleId2desc[e] = n, n;
        }
        getRule(t) {
            return this._ruleId2desc[t];
        }
        getExternalGrammar(t, e) {
            if (this._includedGrammars[t]) return this._includedGrammars[t];
            if (this._grammarRepository) {
                const n = this._grammarRepository.lookup(t);
                if (n) return this._includedGrammars[t] = $n(n, e && e.$base), this._includedGrammars[t];
            }
        }
        tokenizeLine(t, e, n = 0) {
            const r = this._tokenize(t, e, !1, n);
            return {
                tokens: r.lineTokens.getResult(r.ruleStack, r.lineLength),
                ruleStack: r.ruleStack,
                stoppedEarly: r.stoppedEarly
            };
        }
        tokenizeLine2(t, e, n = 0) {
            const r = this._tokenize(t, e, !0, n);
            return {
                tokens: r.lineTokens.getBinaryResult(r.ruleStack, r.lineLength),
                ruleStack: r.ruleStack,
                stoppedEarly: r.stoppedEarly
            };
        }
        _tokenize(t, e, n, r) {
            this._rootId === -1 && (this._rootId = Rr.getCompiledRuleId(this._grammar.repository.$self, this, this._grammar.repository), this.getInjections());
            let i;
            if (!e || e === en.NULL) {
                i = !0;
                const c = this._basicScopeAttributesProvider.getDefaultAttributes(), d = this.themeProvider.getDefaults(), _ = ke.set(0, c.languageId, c.tokenType, null, d.fontStyle, d.foregroundId, d.backgroundId), p = this.getRule(this._rootId).getName(null, null);
                let f;
                p ? f = Ge.createRootAndLookUpScopeName(p, _, this) : f = Ge.createRoot("unknown", _), e = new en(null, this._rootId, -1, -1, !1, null, f, f);
            } else i = !1, e.reset();
            t = t + `
`;
            const o = this.createOnigString(t), s = o.content.length, a = new Ia(n, t, this._tokenTypeMatchers, this.balancedBracketSelectors), l = Pr(this, o, i, 0, e, a, !0, r);
            return Sr(o), {
                lineLength: s,
                lineTokens: a,
                ruleStack: l.stack,
                stoppedEarly: l.stoppedEarly
            };
        }
    };
    function $n(t, e) {
        return t = Ks(t), t.repository = t.repository || {}, t.repository.$self = {
            $vscodeTextmateLocation: t.$vscodeTextmateLocation,
            patterns: t.patterns,
            name: t.scopeName
        }, t.repository.$base = e || t.repository.$self, t;
    }
    var Ge = class re {
        constructor(e, n, r){
            this.parent = e, this.scopePath = n, this.tokenAttributes = r;
        }
        static fromExtension(e, n) {
            let r = e, i = e?.scopePath ?? null;
            for (const o of n)i = Bt.push(i, o.scopeNames), r = new re(r, i, o.encodedTokenAttributes);
            return r;
        }
        static createRoot(e, n) {
            return new re(null, new Bt(null, e), n);
        }
        static createRootAndLookUpScopeName(e, n, r) {
            const i = r.getMetadataForScope(e), o = new Bt(null, e), s = r.themeProvider.themeMatch(o), a = re.mergeAttributes(n, i, s);
            return new re(null, o, a);
        }
        get scopeName() {
            return this.scopePath.scopeName;
        }
        toString() {
            return this.getScopeNames().join(" ");
        }
        equals(e) {
            return re.equals(this, e);
        }
        static equals(e, n) {
            do {
                if (e === n || !e && !n) return !0;
                if (!e || !n || e.scopeName !== n.scopeName || e.tokenAttributes !== n.tokenAttributes) return !1;
                e = e.parent, n = n.parent;
            }while (!0);
        }
        static mergeAttributes(e, n, r) {
            let i = -1, o = 0, s = 0;
            return r !== null && (i = r.fontStyle, o = r.foregroundId, s = r.backgroundId), ke.set(e, n.languageId, n.tokenType, null, i, o, s);
        }
        pushAttributed(e, n) {
            if (e === null) return this;
            if (e.indexOf(" ") === -1) return re._pushAttributed(this, e, n);
            const r = e.split(/ /g);
            let i = this;
            for (const o of r)i = re._pushAttributed(i, o, n);
            return i;
        }
        static _pushAttributed(e, n, r) {
            const i = r.getMetadataForScope(n), o = e.scopePath.push(n), s = r.themeProvider.themeMatch(o), a = re.mergeAttributes(e.tokenAttributes, i, s);
            return new re(e, o, a);
        }
        getScopeNames() {
            return this.scopePath.getSegments();
        }
        getExtensionIfDefined(e) {
            const n = [];
            let r = this;
            for(; r && r !== e;)n.push({
                encodedTokenAttributes: r.tokenAttributes,
                scopeNames: r.scopePath.getExtensionIfDefined(r.parent?.scopePath ?? null)
            }), r = r.parent;
            return r === e ? n.reverse() : void 0;
        }
    }, en = class pe {
        constructor(e, n, r, i, o, s, a, l){
            this.parent = e, this.ruleId = n, this.beginRuleCapturedEOL = o, this.endRule = s, this.nameScopesList = a, this.contentNameScopesList = l, this.depth = this.parent ? this.parent.depth + 1 : 1, this._enterPos = r, this._anchorPos = i;
        }
        _stackElementBrand = void 0;
        static NULL = new pe(null, 0, 0, 0, !1, null, null, null);
        _enterPos;
        _anchorPos;
        depth;
        equals(e) {
            return e === null ? !1 : pe._equals(this, e);
        }
        static _equals(e, n) {
            return e === n ? !0 : this._structuralEquals(e, n) ? Ge.equals(e.contentNameScopesList, n.contentNameScopesList) : !1;
        }
        static _structuralEquals(e, n) {
            do {
                if (e === n || !e && !n) return !0;
                if (!e || !n || e.depth !== n.depth || e.ruleId !== n.ruleId || e.endRule !== n.endRule) return !1;
                e = e.parent, n = n.parent;
            }while (!0);
        }
        clone() {
            return this;
        }
        static _reset(e) {
            for(; e;)e._enterPos = -1, e._anchorPos = -1, e = e.parent;
        }
        reset() {
            pe._reset(this);
        }
        pop() {
            return this.parent;
        }
        safePop() {
            return this.parent ? this.parent : this;
        }
        push(e, n, r, i, o, s, a) {
            return new pe(this, e, n, r, i, o, s, a);
        }
        getEnterPos() {
            return this._enterPos;
        }
        getAnchorPos() {
            return this._anchorPos;
        }
        getRule(e) {
            return e.getRule(this.ruleId);
        }
        toString() {
            const e = [];
            return this._writeString(e, 0), "[" + e.join(",") + "]";
        }
        _writeString(e, n) {
            return this.parent && (n = this.parent._writeString(e, n)), e[n++] = `(${this.ruleId}, ${this.nameScopesList?.toString()}, ${this.contentNameScopesList?.toString()})`, n;
        }
        withContentNameScopesList(e) {
            return this.contentNameScopesList === e ? this : this.parent.push(this.ruleId, this._enterPos, this._anchorPos, this.beginRuleCapturedEOL, this.endRule, this.nameScopesList, e);
        }
        withEndRule(e) {
            return this.endRule === e ? this : new pe(this.parent, this.ruleId, this._enterPos, this._anchorPos, this.beginRuleCapturedEOL, e, this.nameScopesList, this.contentNameScopesList);
        }
        hasSameRuleAs(e) {
            let n = this;
            for(; n && n._enterPos === e._enterPos;){
                if (n.ruleId === e.ruleId) return !0;
                n = n.parent;
            }
            return !1;
        }
        toStateStackFrame() {
            return {
                ruleId: this.ruleId,
                beginRuleCapturedEOL: this.beginRuleCapturedEOL,
                endRule: this.endRule,
                nameScopesList: this.nameScopesList?.getExtensionIfDefined(this.parent?.nameScopesList ?? null) ?? [],
                contentNameScopesList: this.contentNameScopesList?.getExtensionIfDefined(this.nameScopesList) ?? []
            };
        }
        static pushFrame(e, n) {
            const r = Ge.fromExtension(e?.nameScopesList ?? null, n.nameScopesList);
            return new pe(e, n.ruleId, n.enterPos ?? -1, n.anchorPos ?? -1, n.beginRuleCapturedEOL, n.endRule, r, Ge.fromExtension(r, n.contentNameScopesList));
        }
    }, Ca = class {
        balancedBracketScopes;
        unbalancedBracketScopes;
        allowAny = !1;
        constructor(t, e){
            this.balancedBracketScopes = t.flatMap((n)=>n === "*" ? (this.allowAny = !0, []) : gt(n, wt).map((r)=>r.matcher)), this.unbalancedBracketScopes = e.flatMap((n)=>gt(n, wt).map((r)=>r.matcher));
        }
        get matchesAlways() {
            return this.allowAny && this.unbalancedBracketScopes.length === 0;
        }
        get matchesNever() {
            return this.balancedBracketScopes.length === 0 && !this.allowAny;
        }
        match(t) {
            for (const e of this.unbalancedBracketScopes)if (e(t)) return !1;
            for (const e of this.balancedBracketScopes)if (e(t)) return !0;
            return this.allowAny;
        }
    }, Ia = class {
        constructor(t, e, n, r){
            this.balancedBracketSelectors = r, this._emitBinaryTokens = t, this._tokenTypeOverrides = n, this._lineText = null, this._tokens = [], this._binaryTokens = [], this._lastTokenEndIndex = 0;
        }
        _emitBinaryTokens;
        _lineText;
        _tokens;
        _binaryTokens;
        _lastTokenEndIndex;
        _tokenTypeOverrides;
        produce(t, e) {
            this.produceFromScopes(t.contentNameScopesList, e);
        }
        produceFromScopes(t, e) {
            if (this._lastTokenEndIndex >= e) return;
            if (this._emitBinaryTokens) {
                let r = t?.tokenAttributes ?? 0, i = !1;
                if (this.balancedBracketSelectors?.matchesAlways && (i = !0), this._tokenTypeOverrides.length > 0 || this.balancedBracketSelectors && !this.balancedBracketSelectors.matchesAlways && !this.balancedBracketSelectors.matchesNever) {
                    const o = t?.getScopeNames() ?? [];
                    for (const s of this._tokenTypeOverrides)s.matcher(o) && (r = ke.set(r, 0, s.type, null, -1, 0, 0));
                    this.balancedBracketSelectors && (i = this.balancedBracketSelectors.match(o));
                }
                if (i && (r = ke.set(r, 0, 8, i, -1, 0, 0)), this._binaryTokens.length > 0 && this._binaryTokens[this._binaryTokens.length - 1] === r) {
                    this._lastTokenEndIndex = e;
                    return;
                }
                this._binaryTokens.push(this._lastTokenEndIndex), this._binaryTokens.push(r), this._lastTokenEndIndex = e;
                return;
            }
            const n = t?.getScopeNames() ?? [];
            this._tokens.push({
                startIndex: this._lastTokenEndIndex,
                endIndex: e,
                scopes: n
            }), this._lastTokenEndIndex = e;
        }
        getResult(t, e) {
            return this._tokens.length > 0 && this._tokens[this._tokens.length - 1].startIndex === e - 1 && this._tokens.pop(), this._tokens.length === 0 && (this._lastTokenEndIndex = -1, this.produce(t, e), this._tokens[this._tokens.length - 1].startIndex = 0), this._tokens;
        }
        getBinaryResult(t, e) {
            this._binaryTokens.length > 0 && this._binaryTokens[this._binaryTokens.length - 2] === e - 1 && (this._binaryTokens.pop(), this._binaryTokens.pop()), this._binaryTokens.length === 0 && (this._lastTokenEndIndex = -1, this.produce(t, e), this._binaryTokens[this._binaryTokens.length - 2] = 0);
            const n = new Uint32Array(this._binaryTokens.length);
            for(let r = 0, i = this._binaryTokens.length; r < i; r++)n[r] = this._binaryTokens[r];
            return n;
        }
    }, Oa = class {
        constructor(t, e){
            this._onigLib = e, this._theme = t;
        }
        _grammars = new Map;
        _rawGrammars = new Map;
        _injectionGrammars = new Map;
        _theme;
        dispose() {
            for (const t of this._grammars.values())t.dispose();
        }
        setTheme(t) {
            this._theme = t;
        }
        getColorMap() {
            return this._theme.getColorMap();
        }
        addGrammar(t, e) {
            this._rawGrammars.set(t.scopeName, t), e && this._injectionGrammars.set(t.scopeName, e);
        }
        lookup(t) {
            return this._rawGrammars.get(t);
        }
        injections(t) {
            return this._injectionGrammars.get(t);
        }
        getDefaults() {
            return this._theme.getDefaults();
        }
        themeMatch(t) {
            return this._theme.match(t);
        }
        grammarForScopeName(t, e, n, r, i) {
            if (!this._grammars.has(t)) {
                let o = this._rawGrammars.get(t);
                if (!o) return null;
                this._grammars.set(t, La(t, o, e, n, r, i, this, this._onigLib));
            }
            return this._grammars.get(t);
        }
    }, xa = class {
        _options;
        _syncRegistry;
        _ensureGrammarCache;
        constructor(e){
            this._options = e, this._syncRegistry = new Oa(mt.createFromRawTheme(e.theme, e.colorMap), e.onigLib), this._ensureGrammarCache = new Map;
        }
        dispose() {
            this._syncRegistry.dispose();
        }
        setTheme(e, n) {
            this._syncRegistry.setTheme(mt.createFromRawTheme(e, n));
        }
        getColorMap() {
            return this._syncRegistry.getColorMap();
        }
        loadGrammarWithEmbeddedLanguages(e, n, r) {
            return this.loadGrammarWithConfiguration(e, n, {
                embeddedLanguages: r
            });
        }
        loadGrammarWithConfiguration(e, n, r) {
            return this._loadGrammar(e, n, r.embeddedLanguages, r.tokenTypes, new Ca(r.balancedBracketSelectors || [], r.unbalancedBracketSelectors || []));
        }
        loadGrammar(e) {
            return this._loadGrammar(e, 0, null, null, null);
        }
        _loadGrammar(e, n, r, i, o) {
            const s = new la(this._syncRegistry, e);
            for(; s.Q.length > 0;)s.Q.map((a)=>this._loadSingleGrammar(a.scopeName)), s.processQueue();
            return this._grammarForScopeName(e, n, r, i, o);
        }
        _loadSingleGrammar(e) {
            this._ensureGrammarCache.has(e) || (this._doLoadSingleGrammar(e), this._ensureGrammarCache.set(e, !0));
        }
        _doLoadSingleGrammar(e) {
            const n = this._options.loadGrammar(e);
            if (n) {
                const r = typeof this._options.getInjections == "function" ? this._options.getInjections(e) : void 0;
                this._syncRegistry.addGrammar(n, r);
            }
        }
        addGrammar(e, n = [], r = 0, i = null) {
            return this._syncRegistry.addGrammar(e, n), this._grammarForScopeName(e.scopeName, r, i);
        }
        _grammarForScopeName(e, n = 0, r = null, i = null, o = null) {
            return this._syncRegistry.grammarForScopeName(e, n, r, i, o);
        }
    }, tn = en.NULL;
    const Da = [
        "area",
        "base",
        "basefont",
        "bgsound",
        "br",
        "col",
        "command",
        "embed",
        "frame",
        "hr",
        "image",
        "img",
        "input",
        "keygen",
        "link",
        "meta",
        "param",
        "source",
        "track",
        "wbr"
    ];
    class Ye {
        constructor(e, n, r){
            this.normal = n, this.property = e, r && (this.space = r);
        }
    }
    Ye.prototype.normal = {};
    Ye.prototype.property = {};
    Ye.prototype.space = void 0;
    function Cr(t, e) {
        const n = {}, r = {};
        for (const i of t)Object.assign(n, i.property), Object.assign(r, i.normal);
        return new Ye(n, r, e);
    }
    function nn(t) {
        return t.toLowerCase();
    }
    class z {
        constructor(e, n){
            this.attribute = n, this.property = e;
        }
    }
    z.prototype.attribute = "";
    z.prototype.booleanish = !1;
    z.prototype.boolean = !1;
    z.prototype.commaOrSpaceSeparated = !1;
    z.prototype.commaSeparated = !1;
    z.prototype.defined = !1;
    z.prototype.mustUseProperty = !1;
    z.prototype.number = !1;
    z.prototype.overloadedBoolean = !1;
    z.prototype.property = "";
    z.prototype.spaceSeparated = !1;
    z.prototype.space = void 0;
    let Na = 0;
    const T = ye(), D = ye(), rn = ye(), b = ye(), C = ye(), Te = ye(), K = ye();
    function ye() {
        return 2 ** ++Na;
    }
    const on = Object.freeze(Object.defineProperty({
        __proto__: null,
        boolean: T,
        booleanish: D,
        commaOrSpaceSeparated: K,
        commaSeparated: Te,
        number: b,
        overloadedBoolean: rn,
        spaceSeparated: C
    }, Symbol.toStringTag, {
        value: "Module"
    })), Ut = Object.keys(on);
    class un extends z {
        constructor(e, n, r, i){
            let o = -1;
            if (super(e, n), jn(this, "space", i), typeof r == "number") for(; ++o < Ut.length;){
                const s = Ut[o];
                jn(this, Ut[o], (r & on[s]) === on[s]);
            }
        }
    }
    un.prototype.defined = !0;
    function jn(t, e, n) {
        n && (t[e] = n);
    }
    function Ie(t) {
        const e = {}, n = {};
        for (const [r, i] of Object.entries(t.properties)){
            const o = new un(r, t.transform(t.attributes || {}, r), i, t.space);
            t.mustUseProperty && t.mustUseProperty.includes(r) && (o.mustUseProperty = !0), e[r] = o, n[nn(r)] = r, n[nn(o.attribute)] = r;
        }
        return new Ye(e, n, t.space);
    }
    const Ir = Ie({
        properties: {
            ariaActiveDescendant: null,
            ariaAtomic: D,
            ariaAutoComplete: null,
            ariaBusy: D,
            ariaChecked: D,
            ariaColCount: b,
            ariaColIndex: b,
            ariaColSpan: b,
            ariaControls: C,
            ariaCurrent: null,
            ariaDescribedBy: C,
            ariaDetails: null,
            ariaDisabled: D,
            ariaDropEffect: C,
            ariaErrorMessage: null,
            ariaExpanded: D,
            ariaFlowTo: C,
            ariaGrabbed: D,
            ariaHasPopup: null,
            ariaHidden: D,
            ariaInvalid: null,
            ariaKeyShortcuts: null,
            ariaLabel: null,
            ariaLabelledBy: C,
            ariaLevel: b,
            ariaLive: null,
            ariaModal: D,
            ariaMultiLine: D,
            ariaMultiSelectable: D,
            ariaOrientation: null,
            ariaOwns: C,
            ariaPlaceholder: null,
            ariaPosInSet: b,
            ariaPressed: D,
            ariaReadOnly: D,
            ariaRelevant: null,
            ariaRequired: D,
            ariaRoleDescription: C,
            ariaRowCount: b,
            ariaRowIndex: b,
            ariaRowSpan: b,
            ariaSelected: D,
            ariaSetSize: b,
            ariaSort: null,
            ariaValueMax: b,
            ariaValueMin: b,
            ariaValueNow: b,
            ariaValueText: null,
            role: null
        },
        transform (t, e) {
            return e === "role" ? e : "aria-" + e.slice(4).toLowerCase();
        }
    });
    function Or(t, e) {
        return e in t ? t[e] : e;
    }
    function xr(t, e) {
        return Or(t, e.toLowerCase());
    }
    const Va = Ie({
        attributes: {
            acceptcharset: "accept-charset",
            classname: "class",
            htmlfor: "for",
            httpequiv: "http-equiv"
        },
        mustUseProperty: [
            "checked",
            "multiple",
            "muted",
            "selected"
        ],
        properties: {
            abbr: null,
            accept: Te,
            acceptCharset: C,
            accessKey: C,
            action: null,
            allow: null,
            allowFullScreen: T,
            allowPaymentRequest: T,
            allowUserMedia: T,
            alt: null,
            as: null,
            async: T,
            autoCapitalize: null,
            autoComplete: C,
            autoFocus: T,
            autoPlay: T,
            blocking: C,
            capture: null,
            charSet: null,
            checked: T,
            cite: null,
            className: C,
            cols: b,
            colSpan: null,
            content: null,
            contentEditable: D,
            controls: T,
            controlsList: C,
            coords: b | Te,
            crossOrigin: null,
            data: null,
            dateTime: null,
            decoding: null,
            default: T,
            defer: T,
            dir: null,
            dirName: null,
            disabled: T,
            download: rn,
            draggable: D,
            encType: null,
            enterKeyHint: null,
            fetchPriority: null,
            form: null,
            formAction: null,
            formEncType: null,
            formMethod: null,
            formNoValidate: T,
            formTarget: null,
            headers: C,
            height: b,
            hidden: rn,
            high: b,
            href: null,
            hrefLang: null,
            htmlFor: C,
            httpEquiv: C,
            id: null,
            imageSizes: null,
            imageSrcSet: null,
            inert: T,
            inputMode: null,
            integrity: null,
            is: null,
            isMap: T,
            itemId: null,
            itemProp: C,
            itemRef: C,
            itemScope: T,
            itemType: C,
            kind: null,
            label: null,
            lang: null,
            language: null,
            list: null,
            loading: null,
            loop: T,
            low: b,
            manifest: null,
            max: null,
            maxLength: b,
            media: null,
            method: null,
            min: null,
            minLength: b,
            multiple: T,
            muted: T,
            name: null,
            nonce: null,
            noModule: T,
            noValidate: T,
            onAbort: null,
            onAfterPrint: null,
            onAuxClick: null,
            onBeforeMatch: null,
            onBeforePrint: null,
            onBeforeToggle: null,
            onBeforeUnload: null,
            onBlur: null,
            onCancel: null,
            onCanPlay: null,
            onCanPlayThrough: null,
            onChange: null,
            onClick: null,
            onClose: null,
            onContextLost: null,
            onContextMenu: null,
            onContextRestored: null,
            onCopy: null,
            onCueChange: null,
            onCut: null,
            onDblClick: null,
            onDrag: null,
            onDragEnd: null,
            onDragEnter: null,
            onDragExit: null,
            onDragLeave: null,
            onDragOver: null,
            onDragStart: null,
            onDrop: null,
            onDurationChange: null,
            onEmptied: null,
            onEnded: null,
            onError: null,
            onFocus: null,
            onFormData: null,
            onHashChange: null,
            onInput: null,
            onInvalid: null,
            onKeyDown: null,
            onKeyPress: null,
            onKeyUp: null,
            onLanguageChange: null,
            onLoad: null,
            onLoadedData: null,
            onLoadedMetadata: null,
            onLoadEnd: null,
            onLoadStart: null,
            onMessage: null,
            onMessageError: null,
            onMouseDown: null,
            onMouseEnter: null,
            onMouseLeave: null,
            onMouseMove: null,
            onMouseOut: null,
            onMouseOver: null,
            onMouseUp: null,
            onOffline: null,
            onOnline: null,
            onPageHide: null,
            onPageShow: null,
            onPaste: null,
            onPause: null,
            onPlay: null,
            onPlaying: null,
            onPopState: null,
            onProgress: null,
            onRateChange: null,
            onRejectionHandled: null,
            onReset: null,
            onResize: null,
            onScroll: null,
            onScrollEnd: null,
            onSecurityPolicyViolation: null,
            onSeeked: null,
            onSeeking: null,
            onSelect: null,
            onSlotChange: null,
            onStalled: null,
            onStorage: null,
            onSubmit: null,
            onSuspend: null,
            onTimeUpdate: null,
            onToggle: null,
            onUnhandledRejection: null,
            onUnload: null,
            onVolumeChange: null,
            onWaiting: null,
            onWheel: null,
            open: T,
            optimum: b,
            pattern: null,
            ping: C,
            placeholder: null,
            playsInline: T,
            popover: null,
            popoverTarget: null,
            popoverTargetAction: null,
            poster: null,
            preload: null,
            readOnly: T,
            referrerPolicy: null,
            rel: C,
            required: T,
            reversed: T,
            rows: b,
            rowSpan: b,
            sandbox: C,
            scope: null,
            scoped: T,
            seamless: T,
            selected: T,
            shadowRootClonable: T,
            shadowRootDelegatesFocus: T,
            shadowRootMode: null,
            shape: null,
            size: b,
            sizes: null,
            slot: null,
            span: b,
            spellCheck: D,
            src: null,
            srcDoc: null,
            srcLang: null,
            srcSet: null,
            start: b,
            step: null,
            style: null,
            tabIndex: b,
            target: null,
            title: null,
            translate: null,
            type: null,
            typeMustMatch: T,
            useMap: null,
            value: D,
            width: b,
            wrap: null,
            writingSuggestions: null,
            align: null,
            aLink: null,
            archive: C,
            axis: null,
            background: null,
            bgColor: null,
            border: b,
            borderColor: null,
            bottomMargin: b,
            cellPadding: null,
            cellSpacing: null,
            char: null,
            charOff: null,
            classId: null,
            clear: null,
            code: null,
            codeBase: null,
            codeType: null,
            color: null,
            compact: T,
            declare: T,
            event: null,
            face: null,
            frame: null,
            frameBorder: null,
            hSpace: b,
            leftMargin: b,
            link: null,
            longDesc: null,
            lowSrc: null,
            marginHeight: b,
            marginWidth: b,
            noResize: T,
            noHref: T,
            noShade: T,
            noWrap: T,
            object: null,
            profile: null,
            prompt: null,
            rev: null,
            rightMargin: b,
            rules: null,
            scheme: null,
            scrolling: D,
            standby: null,
            summary: null,
            text: null,
            topMargin: b,
            valueType: null,
            version: null,
            vAlign: null,
            vLink: null,
            vSpace: b,
            allowTransparency: null,
            autoCorrect: null,
            autoSave: null,
            disablePictureInPicture: T,
            disableRemotePlayback: T,
            prefix: null,
            property: null,
            results: b,
            security: null,
            unselectable: null
        },
        space: "html",
        transform: xr
    }), Ma = Ie({
        attributes: {
            accentHeight: "accent-height",
            alignmentBaseline: "alignment-baseline",
            arabicForm: "arabic-form",
            baselineShift: "baseline-shift",
            capHeight: "cap-height",
            className: "class",
            clipPath: "clip-path",
            clipRule: "clip-rule",
            colorInterpolation: "color-interpolation",
            colorInterpolationFilters: "color-interpolation-filters",
            colorProfile: "color-profile",
            colorRendering: "color-rendering",
            crossOrigin: "crossorigin",
            dataType: "datatype",
            dominantBaseline: "dominant-baseline",
            enableBackground: "enable-background",
            fillOpacity: "fill-opacity",
            fillRule: "fill-rule",
            floodColor: "flood-color",
            floodOpacity: "flood-opacity",
            fontFamily: "font-family",
            fontSize: "font-size",
            fontSizeAdjust: "font-size-adjust",
            fontStretch: "font-stretch",
            fontStyle: "font-style",
            fontVariant: "font-variant",
            fontWeight: "font-weight",
            glyphName: "glyph-name",
            glyphOrientationHorizontal: "glyph-orientation-horizontal",
            glyphOrientationVertical: "glyph-orientation-vertical",
            hrefLang: "hreflang",
            horizAdvX: "horiz-adv-x",
            horizOriginX: "horiz-origin-x",
            horizOriginY: "horiz-origin-y",
            imageRendering: "image-rendering",
            letterSpacing: "letter-spacing",
            lightingColor: "lighting-color",
            markerEnd: "marker-end",
            markerMid: "marker-mid",
            markerStart: "marker-start",
            navDown: "nav-down",
            navDownLeft: "nav-down-left",
            navDownRight: "nav-down-right",
            navLeft: "nav-left",
            navNext: "nav-next",
            navPrev: "nav-prev",
            navRight: "nav-right",
            navUp: "nav-up",
            navUpLeft: "nav-up-left",
            navUpRight: "nav-up-right",
            onAbort: "onabort",
            onActivate: "onactivate",
            onAfterPrint: "onafterprint",
            onBeforePrint: "onbeforeprint",
            onBegin: "onbegin",
            onCancel: "oncancel",
            onCanPlay: "oncanplay",
            onCanPlayThrough: "oncanplaythrough",
            onChange: "onchange",
            onClick: "onclick",
            onClose: "onclose",
            onCopy: "oncopy",
            onCueChange: "oncuechange",
            onCut: "oncut",
            onDblClick: "ondblclick",
            onDrag: "ondrag",
            onDragEnd: "ondragend",
            onDragEnter: "ondragenter",
            onDragExit: "ondragexit",
            onDragLeave: "ondragleave",
            onDragOver: "ondragover",
            onDragStart: "ondragstart",
            onDrop: "ondrop",
            onDurationChange: "ondurationchange",
            onEmptied: "onemptied",
            onEnd: "onend",
            onEnded: "onended",
            onError: "onerror",
            onFocus: "onfocus",
            onFocusIn: "onfocusin",
            onFocusOut: "onfocusout",
            onHashChange: "onhashchange",
            onInput: "oninput",
            onInvalid: "oninvalid",
            onKeyDown: "onkeydown",
            onKeyPress: "onkeypress",
            onKeyUp: "onkeyup",
            onLoad: "onload",
            onLoadedData: "onloadeddata",
            onLoadedMetadata: "onloadedmetadata",
            onLoadStart: "onloadstart",
            onMessage: "onmessage",
            onMouseDown: "onmousedown",
            onMouseEnter: "onmouseenter",
            onMouseLeave: "onmouseleave",
            onMouseMove: "onmousemove",
            onMouseOut: "onmouseout",
            onMouseOver: "onmouseover",
            onMouseUp: "onmouseup",
            onMouseWheel: "onmousewheel",
            onOffline: "onoffline",
            onOnline: "ononline",
            onPageHide: "onpagehide",
            onPageShow: "onpageshow",
            onPaste: "onpaste",
            onPause: "onpause",
            onPlay: "onplay",
            onPlaying: "onplaying",
            onPopState: "onpopstate",
            onProgress: "onprogress",
            onRateChange: "onratechange",
            onRepeat: "onrepeat",
            onReset: "onreset",
            onResize: "onresize",
            onScroll: "onscroll",
            onSeeked: "onseeked",
            onSeeking: "onseeking",
            onSelect: "onselect",
            onShow: "onshow",
            onStalled: "onstalled",
            onStorage: "onstorage",
            onSubmit: "onsubmit",
            onSuspend: "onsuspend",
            onTimeUpdate: "ontimeupdate",
            onToggle: "ontoggle",
            onUnload: "onunload",
            onVolumeChange: "onvolumechange",
            onWaiting: "onwaiting",
            onZoom: "onzoom",
            overlinePosition: "overline-position",
            overlineThickness: "overline-thickness",
            paintOrder: "paint-order",
            panose1: "panose-1",
            pointerEvents: "pointer-events",
            referrerPolicy: "referrerpolicy",
            renderingIntent: "rendering-intent",
            shapeRendering: "shape-rendering",
            stopColor: "stop-color",
            stopOpacity: "stop-opacity",
            strikethroughPosition: "strikethrough-position",
            strikethroughThickness: "strikethrough-thickness",
            strokeDashArray: "stroke-dasharray",
            strokeDashOffset: "stroke-dashoffset",
            strokeLineCap: "stroke-linecap",
            strokeLineJoin: "stroke-linejoin",
            strokeMiterLimit: "stroke-miterlimit",
            strokeOpacity: "stroke-opacity",
            strokeWidth: "stroke-width",
            tabIndex: "tabindex",
            textAnchor: "text-anchor",
            textDecoration: "text-decoration",
            textRendering: "text-rendering",
            transformOrigin: "transform-origin",
            typeOf: "typeof",
            underlinePosition: "underline-position",
            underlineThickness: "underline-thickness",
            unicodeBidi: "unicode-bidi",
            unicodeRange: "unicode-range",
            unitsPerEm: "units-per-em",
            vAlphabetic: "v-alphabetic",
            vHanging: "v-hanging",
            vIdeographic: "v-ideographic",
            vMathematical: "v-mathematical",
            vectorEffect: "vector-effect",
            vertAdvY: "vert-adv-y",
            vertOriginX: "vert-origin-x",
            vertOriginY: "vert-origin-y",
            wordSpacing: "word-spacing",
            writingMode: "writing-mode",
            xHeight: "x-height",
            playbackOrder: "playbackorder",
            timelineBegin: "timelinebegin"
        },
        properties: {
            about: K,
            accentHeight: b,
            accumulate: null,
            additive: null,
            alignmentBaseline: null,
            alphabetic: b,
            amplitude: b,
            arabicForm: null,
            ascent: b,
            attributeName: null,
            attributeType: null,
            azimuth: b,
            bandwidth: null,
            baselineShift: null,
            baseFrequency: null,
            baseProfile: null,
            bbox: null,
            begin: null,
            bias: b,
            by: null,
            calcMode: null,
            capHeight: b,
            className: C,
            clip: null,
            clipPath: null,
            clipPathUnits: null,
            clipRule: null,
            color: null,
            colorInterpolation: null,
            colorInterpolationFilters: null,
            colorProfile: null,
            colorRendering: null,
            content: null,
            contentScriptType: null,
            contentStyleType: null,
            crossOrigin: null,
            cursor: null,
            cx: null,
            cy: null,
            d: null,
            dataType: null,
            defaultAction: null,
            descent: b,
            diffuseConstant: b,
            direction: null,
            display: null,
            dur: null,
            divisor: b,
            dominantBaseline: null,
            download: T,
            dx: null,
            dy: null,
            edgeMode: null,
            editable: null,
            elevation: b,
            enableBackground: null,
            end: null,
            event: null,
            exponent: b,
            externalResourcesRequired: null,
            fill: null,
            fillOpacity: b,
            fillRule: null,
            filter: null,
            filterRes: null,
            filterUnits: null,
            floodColor: null,
            floodOpacity: null,
            focusable: null,
            focusHighlight: null,
            fontFamily: null,
            fontSize: null,
            fontSizeAdjust: null,
            fontStretch: null,
            fontStyle: null,
            fontVariant: null,
            fontWeight: null,
            format: null,
            fr: null,
            from: null,
            fx: null,
            fy: null,
            g1: Te,
            g2: Te,
            glyphName: Te,
            glyphOrientationHorizontal: null,
            glyphOrientationVertical: null,
            glyphRef: null,
            gradientTransform: null,
            gradientUnits: null,
            handler: null,
            hanging: b,
            hatchContentUnits: null,
            hatchUnits: null,
            height: null,
            href: null,
            hrefLang: null,
            horizAdvX: b,
            horizOriginX: b,
            horizOriginY: b,
            id: null,
            ideographic: b,
            imageRendering: null,
            initialVisibility: null,
            in: null,
            in2: null,
            intercept: b,
            k: b,
            k1: b,
            k2: b,
            k3: b,
            k4: b,
            kernelMatrix: K,
            kernelUnitLength: null,
            keyPoints: null,
            keySplines: null,
            keyTimes: null,
            kerning: null,
            lang: null,
            lengthAdjust: null,
            letterSpacing: null,
            lightingColor: null,
            limitingConeAngle: b,
            local: null,
            markerEnd: null,
            markerMid: null,
            markerStart: null,
            markerHeight: null,
            markerUnits: null,
            markerWidth: null,
            mask: null,
            maskContentUnits: null,
            maskUnits: null,
            mathematical: null,
            max: null,
            media: null,
            mediaCharacterEncoding: null,
            mediaContentEncodings: null,
            mediaSize: b,
            mediaTime: null,
            method: null,
            min: null,
            mode: null,
            name: null,
            navDown: null,
            navDownLeft: null,
            navDownRight: null,
            navLeft: null,
            navNext: null,
            navPrev: null,
            navRight: null,
            navUp: null,
            navUpLeft: null,
            navUpRight: null,
            numOctaves: null,
            observer: null,
            offset: null,
            onAbort: null,
            onActivate: null,
            onAfterPrint: null,
            onBeforePrint: null,
            onBegin: null,
            onCancel: null,
            onCanPlay: null,
            onCanPlayThrough: null,
            onChange: null,
            onClick: null,
            onClose: null,
            onCopy: null,
            onCueChange: null,
            onCut: null,
            onDblClick: null,
            onDrag: null,
            onDragEnd: null,
            onDragEnter: null,
            onDragExit: null,
            onDragLeave: null,
            onDragOver: null,
            onDragStart: null,
            onDrop: null,
            onDurationChange: null,
            onEmptied: null,
            onEnd: null,
            onEnded: null,
            onError: null,
            onFocus: null,
            onFocusIn: null,
            onFocusOut: null,
            onHashChange: null,
            onInput: null,
            onInvalid: null,
            onKeyDown: null,
            onKeyPress: null,
            onKeyUp: null,
            onLoad: null,
            onLoadedData: null,
            onLoadedMetadata: null,
            onLoadStart: null,
            onMessage: null,
            onMouseDown: null,
            onMouseEnter: null,
            onMouseLeave: null,
            onMouseMove: null,
            onMouseOut: null,
            onMouseOver: null,
            onMouseUp: null,
            onMouseWheel: null,
            onOffline: null,
            onOnline: null,
            onPageHide: null,
            onPageShow: null,
            onPaste: null,
            onPause: null,
            onPlay: null,
            onPlaying: null,
            onPopState: null,
            onProgress: null,
            onRateChange: null,
            onRepeat: null,
            onReset: null,
            onResize: null,
            onScroll: null,
            onSeeked: null,
            onSeeking: null,
            onSelect: null,
            onShow: null,
            onStalled: null,
            onStorage: null,
            onSubmit: null,
            onSuspend: null,
            onTimeUpdate: null,
            onToggle: null,
            onUnload: null,
            onVolumeChange: null,
            onWaiting: null,
            onZoom: null,
            opacity: null,
            operator: null,
            order: null,
            orient: null,
            orientation: null,
            origin: null,
            overflow: null,
            overlay: null,
            overlinePosition: b,
            overlineThickness: b,
            paintOrder: null,
            panose1: null,
            path: null,
            pathLength: b,
            patternContentUnits: null,
            patternTransform: null,
            patternUnits: null,
            phase: null,
            ping: C,
            pitch: null,
            playbackOrder: null,
            pointerEvents: null,
            points: null,
            pointsAtX: b,
            pointsAtY: b,
            pointsAtZ: b,
            preserveAlpha: null,
            preserveAspectRatio: null,
            primitiveUnits: null,
            propagate: null,
            property: K,
            r: null,
            radius: null,
            referrerPolicy: null,
            refX: null,
            refY: null,
            rel: K,
            rev: K,
            renderingIntent: null,
            repeatCount: null,
            repeatDur: null,
            requiredExtensions: K,
            requiredFeatures: K,
            requiredFonts: K,
            requiredFormats: K,
            resource: null,
            restart: null,
            result: null,
            rotate: null,
            rx: null,
            ry: null,
            scale: null,
            seed: null,
            shapeRendering: null,
            side: null,
            slope: null,
            snapshotTime: null,
            specularConstant: b,
            specularExponent: b,
            spreadMethod: null,
            spacing: null,
            startOffset: null,
            stdDeviation: null,
            stemh: null,
            stemv: null,
            stitchTiles: null,
            stopColor: null,
            stopOpacity: null,
            strikethroughPosition: b,
            strikethroughThickness: b,
            string: null,
            stroke: null,
            strokeDashArray: K,
            strokeDashOffset: null,
            strokeLineCap: null,
            strokeLineJoin: null,
            strokeMiterLimit: b,
            strokeOpacity: b,
            strokeWidth: null,
            style: null,
            surfaceScale: b,
            syncBehavior: null,
            syncBehaviorDefault: null,
            syncMaster: null,
            syncTolerance: null,
            syncToleranceDefault: null,
            systemLanguage: K,
            tabIndex: b,
            tableValues: null,
            target: null,
            targetX: b,
            targetY: b,
            textAnchor: null,
            textDecoration: null,
            textRendering: null,
            textLength: null,
            timelineBegin: null,
            title: null,
            transformBehavior: null,
            type: null,
            typeOf: K,
            to: null,
            transform: null,
            transformOrigin: null,
            u1: null,
            u2: null,
            underlinePosition: b,
            underlineThickness: b,
            unicode: null,
            unicodeBidi: null,
            unicodeRange: null,
            unitsPerEm: b,
            values: null,
            vAlphabetic: b,
            vMathematical: b,
            vectorEffect: null,
            vHanging: b,
            vIdeographic: b,
            version: null,
            vertAdvY: b,
            vertOriginX: b,
            vertOriginY: b,
            viewBox: null,
            viewTarget: null,
            visibility: null,
            width: null,
            widths: null,
            wordSpacing: null,
            writingMode: null,
            x: null,
            x1: null,
            x2: null,
            xChannelSelector: null,
            xHeight: b,
            y: null,
            y1: null,
            y2: null,
            yChannelSelector: null,
            z: null,
            zoomAndPan: null
        },
        space: "svg",
        transform: Or
    }), Dr = Ie({
        properties: {
            xLinkActuate: null,
            xLinkArcRole: null,
            xLinkHref: null,
            xLinkRole: null,
            xLinkShow: null,
            xLinkTitle: null,
            xLinkType: null
        },
        space: "xlink",
        transform (t, e) {
            return "xlink:" + e.slice(5).toLowerCase();
        }
    }), Nr = Ie({
        attributes: {
            xmlnsxlink: "xmlns:xlink"
        },
        properties: {
            xmlnsXLink: null,
            xmlns: null
        },
        space: "xmlns",
        transform: xr
    }), Vr = Ie({
        properties: {
            xmlBase: null,
            xmlLang: null,
            xmlSpace: null
        },
        space: "xml",
        transform (t, e) {
            return "xml:" + e.slice(3).toLowerCase();
        }
    }), $a = /[A-Z]/g, Bn = /-[a-z]/g, ja = /^data[-\w.:]+$/i;
    function Ba(t, e) {
        const n = nn(e);
        let r = e, i = z;
        if (n in t.normal) return t.property[t.normal[n]];
        if (n.length > 4 && n.slice(0, 4) === "data" && ja.test(e)) {
            if (e.charAt(4) === "-") {
                const o = e.slice(5).replace(Bn, Ua);
                r = "data" + o.charAt(0).toUpperCase() + o.slice(1);
            } else {
                const o = e.slice(4);
                if (!Bn.test(o)) {
                    let s = o.replace($a, Ga);
                    s.charAt(0) !== "-" && (s = "-" + s), e = "data" + s;
                }
            }
            i = un;
        }
        return new i(r, e);
    }
    function Ga(t) {
        return "-" + t.toLowerCase();
    }
    function Ua(t) {
        return t.charAt(1).toUpperCase();
    }
    const Ha = Cr([
        Ir,
        Va,
        Dr,
        Nr,
        Vr
    ], "html"), Mr = Cr([
        Ir,
        Ma,
        Dr,
        Nr,
        Vr
    ], "svg"), Gn = {}.hasOwnProperty;
    function Fa(t, e) {
        const n = e || {};
        function r(i, ...o) {
            let s = r.invalid;
            const a = r.handlers;
            if (i && Gn.call(i, t)) {
                const l = String(i[t]);
                s = Gn.call(a, l) ? a[l] : r.unknown;
            }
            if (s) return s.call(this, i, ...o);
        }
        return r.handlers = n.handlers || {}, r.invalid = n.invalid, r.unknown = n.unknown, r;
    }
    const Wa = /["&'<>`]/g, qa = /[\uD800-\uDBFF][\uDC00-\uDFFF]/g, za = /[\x01-\t\v\f\x0E-\x1F\x7F\x81\x8D\x8F\x90\x9D\xA0-\uFFFF]/g, Ka = /[|\\{}()[\]^$+*?.]/g, Un = new WeakMap;
    function Ja(t, e) {
        if (t = t.replace(e.subset ? Xa(e.subset) : Wa, r), e.subset || e.escapeOnly) return t;
        return t.replace(qa, n).replace(za, r);
        function n(i, o, s) {
            return e.format((i.charCodeAt(0) - 55296) * 1024 + i.charCodeAt(1) - 56320 + 65536, s.charCodeAt(o + 2), e);
        }
        function r(i, o, s) {
            return e.format(i.charCodeAt(0), s.charCodeAt(o + 1), e);
        }
    }
    function Xa(t) {
        let e = Un.get(t);
        return e || (e = Ya(t), Un.set(t, e)), e;
    }
    function Ya(t) {
        const e = [];
        let n = -1;
        for(; ++n < t.length;)e.push(t[n].replace(Ka, "\\$&"));
        return new RegExp("(?:" + e.join("|") + ")", "g");
    }
    const Qa = /[\dA-Fa-f]/;
    function Za(t, e, n) {
        const r = "&#x" + t.toString(16).toUpperCase();
        return n && e && !Qa.test(String.fromCharCode(e)) ? r : r + ";";
    }
    const el = /\d/;
    function tl(t, e, n) {
        const r = "&#" + String(t);
        return n && e && !el.test(String.fromCharCode(e)) ? r : r + ";";
    }
    const nl = [
        "AElig",
        "AMP",
        "Aacute",
        "Acirc",
        "Agrave",
        "Aring",
        "Atilde",
        "Auml",
        "COPY",
        "Ccedil",
        "ETH",
        "Eacute",
        "Ecirc",
        "Egrave",
        "Euml",
        "GT",
        "Iacute",
        "Icirc",
        "Igrave",
        "Iuml",
        "LT",
        "Ntilde",
        "Oacute",
        "Ocirc",
        "Ograve",
        "Oslash",
        "Otilde",
        "Ouml",
        "QUOT",
        "REG",
        "THORN",
        "Uacute",
        "Ucirc",
        "Ugrave",
        "Uuml",
        "Yacute",
        "aacute",
        "acirc",
        "acute",
        "aelig",
        "agrave",
        "amp",
        "aring",
        "atilde",
        "auml",
        "brvbar",
        "ccedil",
        "cedil",
        "cent",
        "copy",
        "curren",
        "deg",
        "divide",
        "eacute",
        "ecirc",
        "egrave",
        "eth",
        "euml",
        "frac12",
        "frac14",
        "frac34",
        "gt",
        "iacute",
        "icirc",
        "iexcl",
        "igrave",
        "iquest",
        "iuml",
        "laquo",
        "lt",
        "macr",
        "micro",
        "middot",
        "nbsp",
        "not",
        "ntilde",
        "oacute",
        "ocirc",
        "ograve",
        "ordf",
        "ordm",
        "oslash",
        "otilde",
        "ouml",
        "para",
        "plusmn",
        "pound",
        "quot",
        "raquo",
        "reg",
        "sect",
        "shy",
        "sup1",
        "sup2",
        "sup3",
        "szlig",
        "thorn",
        "times",
        "uacute",
        "ucirc",
        "ugrave",
        "uml",
        "uuml",
        "yacute",
        "yen",
        "yuml"
    ], Ht = {
        nbsp: " ",
        iexcl: "¡",
        cent: "¢",
        pound: "£",
        curren: "¤",
        yen: "¥",
        brvbar: "¦",
        sect: "§",
        uml: "¨",
        copy: "©",
        ordf: "ª",
        laquo: "«",
        not: "¬",
        shy: "­",
        reg: "®",
        macr: "¯",
        deg: "°",
        plusmn: "±",
        sup2: "²",
        sup3: "³",
        acute: "´",
        micro: "µ",
        para: "¶",
        middot: "·",
        cedil: "¸",
        sup1: "¹",
        ordm: "º",
        raquo: "»",
        frac14: "¼",
        frac12: "½",
        frac34: "¾",
        iquest: "¿",
        Agrave: "À",
        Aacute: "Á",
        Acirc: "Â",
        Atilde: "Ã",
        Auml: "Ä",
        Aring: "Å",
        AElig: "Æ",
        Ccedil: "Ç",
        Egrave: "È",
        Eacute: "É",
        Ecirc: "Ê",
        Euml: "Ë",
        Igrave: "Ì",
        Iacute: "Í",
        Icirc: "Î",
        Iuml: "Ï",
        ETH: "Ð",
        Ntilde: "Ñ",
        Ograve: "Ò",
        Oacute: "Ó",
        Ocirc: "Ô",
        Otilde: "Õ",
        Ouml: "Ö",
        times: "×",
        Oslash: "Ø",
        Ugrave: "Ù",
        Uacute: "Ú",
        Ucirc: "Û",
        Uuml: "Ü",
        Yacute: "Ý",
        THORN: "Þ",
        szlig: "ß",
        agrave: "à",
        aacute: "á",
        acirc: "â",
        atilde: "ã",
        auml: "ä",
        aring: "å",
        aelig: "æ",
        ccedil: "ç",
        egrave: "è",
        eacute: "é",
        ecirc: "ê",
        euml: "ë",
        igrave: "ì",
        iacute: "í",
        icirc: "î",
        iuml: "ï",
        eth: "ð",
        ntilde: "ñ",
        ograve: "ò",
        oacute: "ó",
        ocirc: "ô",
        otilde: "õ",
        ouml: "ö",
        divide: "÷",
        oslash: "ø",
        ugrave: "ù",
        uacute: "ú",
        ucirc: "û",
        uuml: "ü",
        yacute: "ý",
        thorn: "þ",
        yuml: "ÿ",
        fnof: "ƒ",
        Alpha: "Α",
        Beta: "Β",
        Gamma: "Γ",
        Delta: "Δ",
        Epsilon: "Ε",
        Zeta: "Ζ",
        Eta: "Η",
        Theta: "Θ",
        Iota: "Ι",
        Kappa: "Κ",
        Lambda: "Λ",
        Mu: "Μ",
        Nu: "Ν",
        Xi: "Ξ",
        Omicron: "Ο",
        Pi: "Π",
        Rho: "Ρ",
        Sigma: "Σ",
        Tau: "Τ",
        Upsilon: "Υ",
        Phi: "Φ",
        Chi: "Χ",
        Psi: "Ψ",
        Omega: "Ω",
        alpha: "α",
        beta: "β",
        gamma: "γ",
        delta: "δ",
        epsilon: "ε",
        zeta: "ζ",
        eta: "η",
        theta: "θ",
        iota: "ι",
        kappa: "κ",
        lambda: "λ",
        mu: "μ",
        nu: "ν",
        xi: "ξ",
        omicron: "ο",
        pi: "π",
        rho: "ρ",
        sigmaf: "ς",
        sigma: "σ",
        tau: "τ",
        upsilon: "υ",
        phi: "φ",
        chi: "χ",
        psi: "ψ",
        omega: "ω",
        thetasym: "ϑ",
        upsih: "ϒ",
        piv: "ϖ",
        bull: "•",
        hellip: "…",
        prime: "′",
        Prime: "″",
        oline: "‾",
        frasl: "⁄",
        weierp: "℘",
        image: "ℑ",
        real: "ℜ",
        trade: "™",
        alefsym: "ℵ",
        larr: "←",
        uarr: "↑",
        rarr: "→",
        darr: "↓",
        harr: "↔",
        crarr: "↵",
        lArr: "⇐",
        uArr: "⇑",
        rArr: "⇒",
        dArr: "⇓",
        hArr: "⇔",
        forall: "∀",
        part: "∂",
        exist: "∃",
        empty: "∅",
        nabla: "∇",
        isin: "∈",
        notin: "∉",
        ni: "∋",
        prod: "∏",
        sum: "∑",
        minus: "−",
        lowast: "∗",
        radic: "√",
        prop: "∝",
        infin: "∞",
        ang: "∠",
        and: "∧",
        or: "∨",
        cap: "∩",
        cup: "∪",
        int: "∫",
        there4: "∴",
        sim: "∼",
        cong: "≅",
        asymp: "≈",
        ne: "≠",
        equiv: "≡",
        le: "≤",
        ge: "≥",
        sub: "⊂",
        sup: "⊃",
        nsub: "⊄",
        sube: "⊆",
        supe: "⊇",
        oplus: "⊕",
        otimes: "⊗",
        perp: "⊥",
        sdot: "⋅",
        lceil: "⌈",
        rceil: "⌉",
        lfloor: "⌊",
        rfloor: "⌋",
        lang: "〈",
        rang: "〉",
        loz: "◊",
        spades: "♠",
        clubs: "♣",
        hearts: "♥",
        diams: "♦",
        quot: '"',
        amp: "&",
        lt: "<",
        gt: ">",
        OElig: "Œ",
        oelig: "œ",
        Scaron: "Š",
        scaron: "š",
        Yuml: "Ÿ",
        circ: "ˆ",
        tilde: "˜",
        ensp: " ",
        emsp: " ",
        thinsp: " ",
        zwnj: "‌",
        zwj: "‍",
        lrm: "‎",
        rlm: "‏",
        ndash: "–",
        mdash: "—",
        lsquo: "‘",
        rsquo: "’",
        sbquo: "‚",
        ldquo: "“",
        rdquo: "”",
        bdquo: "„",
        dagger: "†",
        Dagger: "‡",
        permil: "‰",
        lsaquo: "‹",
        rsaquo: "›",
        euro: "€"
    }, rl = [
        "cent",
        "copy",
        "divide",
        "gt",
        "lt",
        "not",
        "para",
        "times"
    ], $r = {}.hasOwnProperty, sn = {};
    let ot;
    for(ot in Ht)$r.call(Ht, ot) && (sn[Ht[ot]] = ot);
    const il = /[^\dA-Za-z]/;
    function ol(t, e, n, r) {
        const i = String.fromCharCode(t);
        if ($r.call(sn, i)) {
            const o = sn[i], s = "&" + o;
            return n && nl.includes(o) && !rl.includes(o) && (!r || e && e !== 61 && il.test(String.fromCharCode(e))) ? s : s + ";";
        }
        return "";
    }
    function sl(t, e, n) {
        let r = Za(t, e, n.omitOptionalSemicolons), i;
        if ((n.useNamedReferences || n.useShortestReferences) && (i = ol(t, e, n.omitOptionalSemicolons, n.attribute)), (n.useShortestReferences || !i) && n.useShortestReferences) {
            const o = tl(t, e, n.omitOptionalSemicolons);
            o.length < r.length && (r = o);
        }
        return i && (!n.useShortestReferences || i.length < r.length) ? i : r;
    }
    function Re(t, e) {
        return Ja(t, Object.assign({
            format: sl
        }, e));
    }
    const al = /^>|^->|<!--|-->|--!>|<!-$/g, ll = [
        ">"
    ], cl = [
        "<",
        ">"
    ];
    function ul(t, e, n, r) {
        return r.settings.bogusComments ? "<?" + Re(t.value, Object.assign({}, r.settings.characterReferences, {
            subset: ll
        })) + ">" : "<!--" + t.value.replace(al, i) + "-->";
        function i(o) {
            return Re(o, Object.assign({}, r.settings.characterReferences, {
                subset: cl
            }));
        }
    }
    function dl(t, e, n, r) {
        return "<!" + (r.settings.upperDoctype ? "DOCTYPE" : "doctype") + (r.settings.tightDoctype ? "" : " ") + "html>";
    }
    function Hn(t, e) {
        const n = String(t);
        if (typeof e != "string") throw new TypeError("Expected character");
        let r = 0, i = n.indexOf(e);
        for(; i !== -1;)r++, i = n.indexOf(e, i + e.length);
        return r;
    }
    function _l(t, e) {
        const n = e || {};
        return (t[t.length - 1] === "" ? [
            ...t,
            ""
        ] : t).join((n.padRight ? " " : "") + "," + (n.padLeft === !1 ? "" : " ")).trim();
    }
    function hl(t) {
        return t.join(" ").trim();
    }
    const fl = /[ \t\n\f\r]/g;
    function dn(t) {
        return typeof t == "object" ? t.type === "text" ? Fn(t.value) : !1 : Fn(t);
    }
    function Fn(t) {
        return t.replace(fl, "") === "";
    }
    const B = Br(1), jr = Br(-1), pl = [];
    function Br(t) {
        return e;
        function e(n, r, i) {
            const o = n ? n.children : pl;
            let s = (r || 0) + t, a = o[s];
            if (!i) for(; a && dn(a);)s += t, a = o[s];
            return a;
        }
    }
    const ml = {}.hasOwnProperty;
    function Gr(t) {
        return e;
        function e(n, r, i) {
            return ml.call(t, n.tagName) && t[n.tagName](n, r, i);
        }
    }
    const _n = Gr({
        body: bl,
        caption: Ft,
        colgroup: Ft,
        dd: vl,
        dt: El,
        head: Ft,
        html: gl,
        li: wl,
        optgroup: Sl,
        option: Al,
        p: yl,
        rp: Wn,
        rt: Wn,
        tbody: Rl,
        td: qn,
        tfoot: Ll,
        th: qn,
        thead: Tl,
        tr: Pl
    });
    function Ft(t, e, n) {
        const r = B(n, e, !0);
        return !r || r.type !== "comment" && !(r.type === "text" && dn(r.value.charAt(0)));
    }
    function gl(t, e, n) {
        const r = B(n, e);
        return !r || r.type !== "comment";
    }
    function bl(t, e, n) {
        const r = B(n, e);
        return !r || r.type !== "comment";
    }
    function yl(t, e, n) {
        const r = B(n, e);
        return r ? r.type === "element" && (r.tagName === "address" || r.tagName === "article" || r.tagName === "aside" || r.tagName === "blockquote" || r.tagName === "details" || r.tagName === "div" || r.tagName === "dl" || r.tagName === "fieldset" || r.tagName === "figcaption" || r.tagName === "figure" || r.tagName === "footer" || r.tagName === "form" || r.tagName === "h1" || r.tagName === "h2" || r.tagName === "h3" || r.tagName === "h4" || r.tagName === "h5" || r.tagName === "h6" || r.tagName === "header" || r.tagName === "hgroup" || r.tagName === "hr" || r.tagName === "main" || r.tagName === "menu" || r.tagName === "nav" || r.tagName === "ol" || r.tagName === "p" || r.tagName === "pre" || r.tagName === "section" || r.tagName === "table" || r.tagName === "ul") : !n || !(n.type === "element" && (n.tagName === "a" || n.tagName === "audio" || n.tagName === "del" || n.tagName === "ins" || n.tagName === "map" || n.tagName === "noscript" || n.tagName === "video"));
    }
    function wl(t, e, n) {
        const r = B(n, e);
        return !r || r.type === "element" && r.tagName === "li";
    }
    function El(t, e, n) {
        const r = B(n, e);
        return !!(r && r.type === "element" && (r.tagName === "dt" || r.tagName === "dd"));
    }
    function vl(t, e, n) {
        const r = B(n, e);
        return !r || r.type === "element" && (r.tagName === "dt" || r.tagName === "dd");
    }
    function Wn(t, e, n) {
        const r = B(n, e);
        return !r || r.type === "element" && (r.tagName === "rp" || r.tagName === "rt");
    }
    function Sl(t, e, n) {
        const r = B(n, e);
        return !r || r.type === "element" && r.tagName === "optgroup";
    }
    function Al(t, e, n) {
        const r = B(n, e);
        return !r || r.type === "element" && (r.tagName === "option" || r.tagName === "optgroup");
    }
    function Tl(t, e, n) {
        const r = B(n, e);
        return !!(r && r.type === "element" && (r.tagName === "tbody" || r.tagName === "tfoot"));
    }
    function Rl(t, e, n) {
        const r = B(n, e);
        return !r || r.type === "element" && (r.tagName === "tbody" || r.tagName === "tfoot");
    }
    function Ll(t, e, n) {
        return !B(n, e);
    }
    function Pl(t, e, n) {
        const r = B(n, e);
        return !r || r.type === "element" && r.tagName === "tr";
    }
    function qn(t, e, n) {
        const r = B(n, e);
        return !r || r.type === "element" && (r.tagName === "td" || r.tagName === "th");
    }
    const kl = Gr({
        body: Ol,
        colgroup: xl,
        head: Il,
        html: Cl,
        tbody: Dl
    });
    function Cl(t) {
        const e = B(t, -1);
        return !e || e.type !== "comment";
    }
    function Il(t) {
        const e = new Set;
        for (const r of t.children)if (r.type === "element" && (r.tagName === "base" || r.tagName === "title")) {
            if (e.has(r.tagName)) return !1;
            e.add(r.tagName);
        }
        const n = t.children[0];
        return !n || n.type === "element";
    }
    function Ol(t) {
        const e = B(t, -1, !0);
        return !e || e.type !== "comment" && !(e.type === "text" && dn(e.value.charAt(0))) && !(e.type === "element" && (e.tagName === "meta" || e.tagName === "link" || e.tagName === "script" || e.tagName === "style" || e.tagName === "template"));
    }
    function xl(t, e, n) {
        const r = jr(n, e), i = B(t, -1, !0);
        return n && r && r.type === "element" && r.tagName === "colgroup" && _n(r, n.children.indexOf(r), n) ? !1 : !!(i && i.type === "element" && i.tagName === "col");
    }
    function Dl(t, e, n) {
        const r = jr(n, e), i = B(t, -1);
        return n && r && r.type === "element" && (r.tagName === "thead" || r.tagName === "tbody") && _n(r, n.children.indexOf(r), n) ? !1 : !!(i && i.type === "element" && i.tagName === "tr");
    }
    const st = {
        name: [
            [
                `	
\f\r &/=>`.split(""),
                `	
\f\r "&'/=>\``.split("")
            ],
            [
                `\0	
\f\r "&'/<=>`.split(""),
                `\0	
\f\r "&'/<=>\``.split("")
            ]
        ],
        unquoted: [
            [
                `	
\f\r &>`.split(""),
                `\0	
\f\r "&'<=>\``.split("")
            ],
            [
                `\0	
\f\r "&'<=>\``.split(""),
                `\0	
\f\r "&'<=>\``.split("")
            ]
        ],
        single: [
            [
                "&'".split(""),
                "\"&'`".split("")
            ],
            [
                "\0&'".split(""),
                "\0\"&'`".split("")
            ]
        ],
        double: [
            [
                '"&'.split(""),
                "\"&'`".split("")
            ],
            [
                '\0"&'.split(""),
                "\0\"&'`".split("")
            ]
        ]
    };
    function Nl(t, e, n, r) {
        const i = r.schema, o = i.space === "svg" ? !1 : r.settings.omitOptionalTags;
        let s = i.space === "svg" ? r.settings.closeEmptyElements : r.settings.voids.includes(t.tagName.toLowerCase());
        const a = [];
        let l;
        i.space === "html" && t.tagName === "svg" && (r.schema = Mr);
        const c = Vl(r, t.properties), d = r.all(i.space === "html" && t.tagName === "template" ? t.content : t);
        return r.schema = i, d && (s = !1), (c || !o || !kl(t, e, n)) && (a.push("<", t.tagName, c ? " " + c : ""), s && (i.space === "svg" || r.settings.closeSelfClosing) && (l = c.charAt(c.length - 1), (!r.settings.tightSelfClosing || l === "/" || l && l !== '"' && l !== "'") && a.push(" "), a.push("/")), a.push(">")), a.push(d), !s && (!o || !_n(t, e, n)) && a.push("</" + t.tagName + ">"), a.join("");
    }
    function Vl(t, e) {
        const n = [];
        let r = -1, i;
        if (e) {
            for(i in e)if (e[i] !== null && e[i] !== void 0) {
                const o = Ml(t, i, e[i]);
                o && n.push(o);
            }
        }
        for(; ++r < n.length;){
            const o = t.settings.tightAttributes ? n[r].charAt(n[r].length - 1) : void 0;
            r !== n.length - 1 && o !== '"' && o !== "'" && (n[r] += " ");
        }
        return n.join("");
    }
    function Ml(t, e, n) {
        const r = Ba(t.schema, e), i = t.settings.allowParseErrors && t.schema.space === "html" ? 0 : 1, o = t.settings.allowDangerousCharacters ? 0 : 1;
        let s = t.quote, a;
        if (r.overloadedBoolean && (n === r.attribute || n === "") ? n = !0 : (r.boolean || r.overloadedBoolean) && (typeof n != "string" || n === r.attribute || n === "") && (n = !!n), n == null || n === !1 || typeof n == "number" && Number.isNaN(n)) return "";
        const l = Re(r.attribute, Object.assign({}, t.settings.characterReferences, {
            subset: st.name[i][o]
        }));
        return n === !0 || (n = Array.isArray(n) ? (r.commaSeparated ? _l : hl)(n, {
            padLeft: !t.settings.tightCommaSeparatedLists
        }) : String(n), t.settings.collapseEmptyAttributes && !n) ? l : (t.settings.preferUnquoted && (a = Re(n, Object.assign({}, t.settings.characterReferences, {
            attribute: !0,
            subset: st.unquoted[i][o]
        }))), a !== n && (t.settings.quoteSmart && Hn(n, s) > Hn(n, t.alternative) && (s = t.alternative), a = s + Re(n, Object.assign({}, t.settings.characterReferences, {
            subset: (s === "'" ? st.single : st.double)[i][o],
            attribute: !0
        })) + s), l + (a && "=" + a));
    }
    const $l = [
        "<",
        "&"
    ];
    function Ur(t, e, n, r) {
        return n && n.type === "element" && (n.tagName === "script" || n.tagName === "style") ? t.value : Re(t.value, Object.assign({}, r.settings.characterReferences, {
            subset: $l
        }));
    }
    function jl(t, e, n, r) {
        return r.settings.allowDangerousHtml ? t.value : Ur(t, e, n, r);
    }
    function Bl(t, e, n, r) {
        return r.all(t);
    }
    const Gl = Fa("type", {
        invalid: Ul,
        unknown: Hl,
        handlers: {
            comment: ul,
            doctype: dl,
            element: Nl,
            raw: jl,
            root: Bl,
            text: Ur
        }
    });
    function Ul(t) {
        throw new Error("Expected node, not `" + t + "`");
    }
    function Hl(t) {
        const e = t;
        throw new Error("Cannot compile unknown node `" + e.type + "`");
    }
    const Fl = {}, Wl = {}, ql = [];
    function zl(t, e) {
        const n = Fl, r = n.quote || '"', i = r === '"' ? "'" : '"';
        if (r !== '"' && r !== "'") throw new Error("Invalid quote `" + r + "`, expected `'` or `\"`");
        return {
            one: Kl,
            all: Jl,
            settings: {
                omitOptionalTags: n.omitOptionalTags || !1,
                allowParseErrors: n.allowParseErrors || !1,
                allowDangerousCharacters: n.allowDangerousCharacters || !1,
                quoteSmart: n.quoteSmart || !1,
                preferUnquoted: n.preferUnquoted || !1,
                tightAttributes: n.tightAttributes || !1,
                upperDoctype: n.upperDoctype || !1,
                tightDoctype: n.tightDoctype || !1,
                bogusComments: n.bogusComments || !1,
                tightCommaSeparatedLists: n.tightCommaSeparatedLists || !1,
                tightSelfClosing: n.tightSelfClosing || !1,
                collapseEmptyAttributes: n.collapseEmptyAttributes || !1,
                allowDangerousHtml: n.allowDangerousHtml || !1,
                voids: n.voids || Da,
                characterReferences: n.characterReferences || Wl,
                closeSelfClosing: n.closeSelfClosing || !1,
                closeEmptyElements: n.closeEmptyElements || !1
            },
            schema: n.space === "svg" ? Mr : Ha,
            quote: r,
            alternative: i
        }.one(Array.isArray(t) ? {
            type: "root",
            children: t
        } : t, void 0, void 0);
    }
    function Kl(t, e, n) {
        return Gl(t, e, n, this);
    }
    function Jl(t) {
        const e = [], n = t && t.children || ql;
        let r = -1;
        for(; ++r < n.length;)e[r] = this.one(n[r], r, t);
        return e.join("");
    }
    function Xl(t) {
        return Array.isArray(t) ? t : [
            t
        ];
    }
    function kt(t, e = !1) {
        const n = t.split(/(\r?\n)/g);
        let r = 0;
        const i = [];
        for(let o = 0; o < n.length; o += 2){
            const s = e ? n[o] + (n[o + 1] || "") : n[o];
            i.push([
                s,
                r
            ]), r += n[o].length, r += n[o + 1]?.length || 0;
        }
        return i;
    }
    function hn(t) {
        return !t || [
            "plaintext",
            "txt",
            "text",
            "plain"
        ].includes(t);
    }
    function Hr(t) {
        return t === "ansi" || hn(t);
    }
    function fn(t) {
        return t === "none";
    }
    function Fr(t) {
        return fn(t);
    }
    function Wr(t, e) {
        if (!e) return t;
        t.properties ||= {}, t.properties.class ||= [], typeof t.properties.class == "string" && (t.properties.class = t.properties.class.split(/\s+/g)), Array.isArray(t.properties.class) || (t.properties.class = []);
        const n = Array.isArray(e) ? e : e.split(/\s+/g);
        for (const r of n)r && !t.properties.class.includes(r) && t.properties.class.push(r);
        return t;
    }
    function Yl(t, e) {
        let n = 0;
        const r = [];
        for (const i of e)i > n && r.push({
            ...t,
            content: t.content.slice(n, i),
            offset: t.offset + n
        }), n = i;
        return n < t.content.length && r.push({
            ...t,
            content: t.content.slice(n),
            offset: t.offset + n
        }), r;
    }
    function Ql(t, e) {
        const n = Array.from(e instanceof Set ? e : new Set(e)).sort((r, i)=>r - i);
        return n.length ? t.map((r)=>r.flatMap((i)=>{
                const o = n.filter((s)=>i.offset < s && s < i.offset + i.content.length).map((s)=>s - i.offset).sort((s, a)=>s - a);
                return o.length ? Yl(i, o) : i;
            })) : t;
    }
    async function qr(t) {
        return Promise.resolve(typeof t == "function" ? t() : t).then((e)=>e.default || e);
    }
    function Et(t, e) {
        const n = typeof t == "string" ? {} : {
            ...t.colorReplacements
        }, r = typeof t == "string" ? t : t.name;
        for (const [i, o] of Object.entries(e?.colorReplacements || {}))typeof o == "string" ? n[i] = o : i === r && Object.assign(n, o);
        return n;
    }
    function me(t, e) {
        return t && (e?.[t?.toLowerCase()] || t);
    }
    function zr(t) {
        const e = {};
        return t.color && (e.color = t.color), t.bgColor && (e["background-color"] = t.bgColor), t.fontStyle && (t.fontStyle & ae.Italic && (e["font-style"] = "italic"), t.fontStyle & ae.Bold && (e["font-weight"] = "bold"), t.fontStyle & ae.Underline && (e["text-decoration"] = "underline")), e;
    }
    function Zl(t) {
        return typeof t == "string" ? t : Object.entries(t).map(([e, n])=>`${e}:${n}`).join(";");
    }
    function ec(t) {
        const e = kt(t, !0).map(([i])=>i);
        function n(i) {
            if (i === t.length) return {
                line: e.length - 1,
                character: e[e.length - 1].length
            };
            let o = i, s = 0;
            for (const a of e){
                if (o < a.length) break;
                o -= a.length, s++;
            }
            return {
                line: s,
                character: o
            };
        }
        function r(i, o) {
            let s = 0;
            for(let a = 0; a < i; a++)s += e[a].length;
            return s += o, s;
        }
        return {
            lines: e,
            indexToPos: n,
            posToIndex: r
        };
    }
    class W extends Error {
        constructor(e){
            super(e), this.name = "ShikiError";
        }
    }
    const Kr = new WeakMap;
    function Ct(t, e) {
        Kr.set(t, e);
    }
    function qe(t) {
        return Kr.get(t);
    }
    class Oe {
        _stacks = {};
        lang;
        get themes() {
            return Object.keys(this._stacks);
        }
        get theme() {
            return this.themes[0];
        }
        get _stack() {
            return this._stacks[this.theme];
        }
        static initial(e, n) {
            return new Oe(Object.fromEntries(Xl(n).map((r)=>[
                    r,
                    tn
                ])), e);
        }
        constructor(...e){
            if (e.length === 2) {
                const [n, r] = e;
                this.lang = r, this._stacks = n;
            } else {
                const [n, r, i] = e;
                this.lang = r, this._stacks = {
                    [i]: n
                };
            }
        }
        getInternalStack(e = this.theme) {
            return this._stacks[e];
        }
        get scopes() {
            return zn(this._stacks[this.theme]);
        }
        getScopes(e = this.theme) {
            return zn(this._stacks[e]);
        }
        toJSON() {
            return {
                lang: this.lang,
                theme: this.theme,
                themes: this.themes,
                scopes: this.scopes
            };
        }
    }
    function zn(t) {
        const e = [], n = new Set;
        function r(i) {
            if (n.has(i)) return;
            n.add(i);
            const o = i?.nameScopesList?.scopeName;
            o && e.push(o), i.parent && r(i.parent);
        }
        return r(t), e;
    }
    function tc(t, e) {
        if (!(t instanceof Oe)) throw new W("Invalid grammar state");
        return t.getInternalStack(e);
    }
    function nc() {
        const t = new WeakMap;
        function e(n) {
            if (!t.has(n.meta)) {
                let r = function(s) {
                    if (typeof s == "number") {
                        if (s < 0 || s > n.source.length) throw new W(`Invalid decoration offset: ${s}. Code length: ${n.source.length}`);
                        return {
                            ...i.indexToPos(s),
                            offset: s
                        };
                    } else {
                        const a = i.lines[s.line];
                        if (a === void 0) throw new W(`Invalid decoration position ${JSON.stringify(s)}. Lines length: ${i.lines.length}`);
                        if (s.character < 0 || s.character > a.length) throw new W(`Invalid decoration position ${JSON.stringify(s)}. Line ${s.line} length: ${a.length}`);
                        return {
                            ...s,
                            offset: i.posToIndex(s.line, s.character)
                        };
                    }
                };
                const i = ec(n.source), o = (n.options.decorations || []).map((s)=>({
                        ...s,
                        start: r(s.start),
                        end: r(s.end)
                    }));
                rc(o), t.set(n.meta, {
                    decorations: o,
                    converter: i,
                    source: n.source
                });
            }
            return t.get(n.meta);
        }
        return {
            name: "shiki:decorations",
            tokens (n) {
                if (!this.options.decorations?.length) return;
                const i = e(this).decorations.flatMap((s)=>[
                        s.start.offset,
                        s.end.offset
                    ]);
                return Ql(n, i);
            },
            code (n) {
                if (!this.options.decorations?.length) return;
                const r = e(this), i = Array.from(n.children).filter((d)=>d.type === "element" && d.tagName === "span");
                if (i.length !== r.converter.lines.length) throw new W(`Number of lines in code element (${i.length}) does not match the number of lines in the source (${r.converter.lines.length}). Failed to apply decorations.`);
                function o(d, _, p, f) {
                    const h = i[d];
                    let E = "", g = -1, w = -1;
                    if (_ === 0 && (g = 0), p === 0 && (w = 0), p === Number.POSITIVE_INFINITY && (w = h.children.length), g === -1 || w === -1) for(let y = 0; y < h.children.length; y++)E += Jr(h.children[y]), g === -1 && E.length === _ && (g = y + 1), w === -1 && E.length === p && (w = y + 1);
                    if (g === -1) throw new W(`Failed to find start index for decoration ${JSON.stringify(f.start)}`);
                    if (w === -1) throw new W(`Failed to find end index for decoration ${JSON.stringify(f.end)}`);
                    const m = h.children.slice(g, w);
                    if (!f.alwaysWrap && m.length === h.children.length) a(h, f, "line");
                    else if (!f.alwaysWrap && m.length === 1 && m[0].type === "element") a(m[0], f, "token");
                    else {
                        const y = {
                            type: "element",
                            tagName: "span",
                            properties: {},
                            children: m
                        };
                        a(y, f, "wrapper"), h.children.splice(g, m.length, y);
                    }
                }
                function s(d, _) {
                    i[d] = a(i[d], _, "line");
                }
                function a(d, _, p) {
                    const f = _.properties || {}, h = _.transform || ((E)=>E);
                    return d.tagName = _.tagName || "span", d.properties = {
                        ...d.properties,
                        ...f,
                        class: d.properties.class
                    }, _.properties?.class && Wr(d, _.properties.class), d = h(d, p) || d, d;
                }
                const l = [], c = r.decorations.sort((d, _)=>_.start.offset - d.start.offset);
                for (const d of c){
                    const { start: _, end: p } = d;
                    if (_.line === p.line) o(_.line, _.character, p.character, d);
                    else if (_.line < p.line) {
                        o(_.line, _.character, Number.POSITIVE_INFINITY, d);
                        for(let f = _.line + 1; f < p.line; f++)l.unshift(()=>s(f, d));
                        o(p.line, 0, p.character, d);
                    }
                }
                l.forEach((d)=>d());
            }
        };
    }
    function rc(t) {
        for(let e = 0; e < t.length; e++){
            const n = t[e];
            if (n.start.offset > n.end.offset) throw new W(`Invalid decoration range: ${JSON.stringify(n.start)} - ${JSON.stringify(n.end)}`);
            for(let r = e + 1; r < t.length; r++){
                const i = t[r], o = n.start.offset < i.start.offset && i.start.offset < n.end.offset, s = n.start.offset < i.end.offset && i.end.offset < n.end.offset, a = i.start.offset < n.start.offset && n.start.offset < i.end.offset, l = i.start.offset < n.end.offset && n.end.offset < i.end.offset;
                if (o || s || a || l) {
                    if (s && s || a && l) continue;
                    throw new W(`Decorations ${JSON.stringify(n.start)} and ${JSON.stringify(i.start)} intersect.`);
                }
            }
        }
    }
    function Jr(t) {
        return t.type === "text" ? t.value : t.type === "element" ? t.children.map(Jr).join("") : "";
    }
    const ic = [
        nc()
    ];
    function vt(t) {
        return [
            ...t.transformers || [],
            ...ic
        ];
    }
    var ge = [
        "black",
        "red",
        "green",
        "yellow",
        "blue",
        "magenta",
        "cyan",
        "white",
        "brightBlack",
        "brightRed",
        "brightGreen",
        "brightYellow",
        "brightBlue",
        "brightMagenta",
        "brightCyan",
        "brightWhite"
    ], Wt = {
        1: "bold",
        2: "dim",
        3: "italic",
        4: "underline",
        7: "reverse",
        9: "strikethrough"
    };
    function oc(t, e) {
        const n = t.indexOf("\x1B[", e);
        if (n !== -1) {
            const r = t.indexOf("m", n);
            return {
                sequence: t.substring(n + 2, r).split(";"),
                startPosition: n,
                position: r + 1
            };
        }
        return {
            position: t.length
        };
    }
    function Kn(t, e) {
        let n = 1;
        const r = t[e + n++];
        let i;
        if (r === "2") {
            const o = [
                t[e + n++],
                t[e + n++],
                t[e + n]
            ].map((s)=>Number.parseInt(s));
            o.length === 3 && !o.some((s)=>Number.isNaN(s)) && (i = {
                type: "rgb",
                rgb: o
            });
        } else if (r === "5") {
            const o = Number.parseInt(t[e + n]);
            Number.isNaN(o) || (i = {
                type: "table",
                index: Number(o)
            });
        }
        return [
            n,
            i
        ];
    }
    function sc(t) {
        const e = [];
        for(let n = 0; n < t.length; n++){
            const r = t[n], i = Number.parseInt(r);
            if (!Number.isNaN(i)) if (i === 0) e.push({
                type: "resetAll"
            });
            else if (i <= 9) Wt[i] && e.push({
                type: "setDecoration",
                value: Wt[i]
            });
            else if (i <= 29) {
                const o = Wt[i - 20];
                o && e.push({
                    type: "resetDecoration",
                    value: o
                });
            } else if (i <= 37) e.push({
                type: "setForegroundColor",
                value: {
                    type: "named",
                    name: ge[i - 30]
                }
            });
            else if (i === 38) {
                const [o, s] = Kn(t, n);
                s && e.push({
                    type: "setForegroundColor",
                    value: s
                }), n += o;
            } else if (i === 39) e.push({
                type: "resetForegroundColor"
            });
            else if (i <= 47) e.push({
                type: "setBackgroundColor",
                value: {
                    type: "named",
                    name: ge[i - 40]
                }
            });
            else if (i === 48) {
                const [o, s] = Kn(t, n);
                s && e.push({
                    type: "setBackgroundColor",
                    value: s
                }), n += o;
            } else i === 49 ? e.push({
                type: "resetBackgroundColor"
            }) : i >= 90 && i <= 97 ? e.push({
                type: "setForegroundColor",
                value: {
                    type: "named",
                    name: ge[i - 90 + 8]
                }
            }) : i >= 100 && i <= 107 && e.push({
                type: "setBackgroundColor",
                value: {
                    type: "named",
                    name: ge[i - 100 + 8]
                }
            });
        }
        return e;
    }
    function ac() {
        let t = null, e = null, n = new Set;
        return {
            parse (r) {
                const i = [];
                let o = 0;
                do {
                    const s = oc(r, o), a = s.sequence ? r.substring(o, s.startPosition) : r.substring(o);
                    if (a.length > 0 && i.push({
                        value: a,
                        foreground: t,
                        background: e,
                        decorations: new Set(n)
                    }), s.sequence) {
                        const l = sc(s.sequence);
                        for (const c of l)c.type === "resetAll" ? (t = null, e = null, n.clear()) : c.type === "resetForegroundColor" ? t = null : c.type === "resetBackgroundColor" ? e = null : c.type === "resetDecoration" && n.delete(c.value);
                        for (const c of l)c.type === "setForegroundColor" ? t = c.value : c.type === "setBackgroundColor" ? e = c.value : c.type === "setDecoration" && n.add(c.value);
                    }
                    o = s.position;
                }while (o < r.length);
                return i;
            }
        };
    }
    var lc = {
        black: "#000000",
        red: "#bb0000",
        green: "#00bb00",
        yellow: "#bbbb00",
        blue: "#0000bb",
        magenta: "#ff00ff",
        cyan: "#00bbbb",
        white: "#eeeeee",
        brightBlack: "#555555",
        brightRed: "#ff5555",
        brightGreen: "#00ff00",
        brightYellow: "#ffff55",
        brightBlue: "#5555ff",
        brightMagenta: "#ff55ff",
        brightCyan: "#55ffff",
        brightWhite: "#ffffff"
    };
    function cc(t = lc) {
        function e(a) {
            return t[a];
        }
        function n(a) {
            return `#${a.map((l)=>Math.max(0, Math.min(l, 255)).toString(16).padStart(2, "0")).join("")}`;
        }
        let r;
        function i() {
            if (r) return r;
            r = [];
            for(let c = 0; c < ge.length; c++)r.push(e(ge[c]));
            let a = [
                0,
                95,
                135,
                175,
                215,
                255
            ];
            for(let c = 0; c < 6; c++)for(let d = 0; d < 6; d++)for(let _ = 0; _ < 6; _++)r.push(n([
                a[c],
                a[d],
                a[_]
            ]));
            let l = 8;
            for(let c = 0; c < 24; c++, l += 10)r.push(n([
                l,
                l,
                l
            ]));
            return r;
        }
        function o(a) {
            return i()[a];
        }
        function s(a) {
            switch(a.type){
                case "named":
                    return e(a.name);
                case "rgb":
                    return n(a.rgb);
                case "table":
                    return o(a.index);
            }
        }
        return {
            value: s
        };
    }
    function uc(t, e, n) {
        const r = Et(t, n), i = kt(e), o = cc(Object.fromEntries(ge.map((a)=>[
                a,
                t.colors?.[`terminal.ansi${a[0].toUpperCase()}${a.substring(1)}`]
            ]))), s = ac();
        return i.map((a)=>s.parse(a[0]).map((l)=>{
                let c, d;
                l.decorations.has("reverse") ? (c = l.background ? o.value(l.background) : t.bg, d = l.foreground ? o.value(l.foreground) : t.fg) : (c = l.foreground ? o.value(l.foreground) : t.fg, d = l.background ? o.value(l.background) : void 0), c = me(c, r), d = me(d, r), l.decorations.has("dim") && (c = dc(c));
                let _ = ae.None;
                return l.decorations.has("bold") && (_ |= ae.Bold), l.decorations.has("italic") && (_ |= ae.Italic), l.decorations.has("underline") && (_ |= ae.Underline), {
                    content: l.value,
                    offset: a[1],
                    color: c,
                    bgColor: d,
                    fontStyle: _
                };
            }));
    }
    function dc(t) {
        const e = t.match(/#([0-9a-f]{3})([0-9a-f]{3})?([0-9a-f]{2})?/);
        if (e) if (e[3]) {
            const r = Math.round(Number.parseInt(e[3], 16) / 2).toString(16).padStart(2, "0");
            return `#${e[1]}${e[2]}${r}`;
        } else return e[2] ? `#${e[1]}${e[2]}80` : `#${Array.from(e[1]).map((r)=>`${r}${r}`).join("")}80`;
        const n = t.match(/var\((--[\w-]+-ansi-[\w-]+)\)/);
        return n ? `var(${n[1]}-dim)` : t;
    }
    function pn(t, e, n = {}) {
        const { lang: r = "text", theme: i = t.getLoadedThemes()[0] } = n;
        if (hn(r) || fn(i)) return kt(e).map((l)=>[
                {
                    content: l[0],
                    offset: l[1]
                }
            ]);
        const { theme: o, colorMap: s } = t.setTheme(i);
        if (r === "ansi") return uc(o, e, n);
        const a = t.getLanguage(r);
        if (n.grammarState) {
            if (n.grammarState.lang !== a.name) throw new ce(`Grammar state language "${n.grammarState.lang}" does not match highlight language "${a.name}"`);
            if (!n.grammarState.themes.includes(o.name)) throw new ce(`Grammar state themes "${n.grammarState.themes}" do not contain highlight theme "${o.name}"`);
        }
        return hc(e, a, o, s, n);
    }
    function _c(...t) {
        if (t.length === 2) return qe(t[1]);
        const [e, n, r = {}] = t, { lang: i = "text", theme: o = e.getLoadedThemes()[0] } = r;
        if (hn(i) || fn(o)) throw new ce("Plain language does not have grammar state");
        if (i === "ansi") throw new ce("ANSI language does not have grammar state");
        const { theme: s, colorMap: a } = e.setTheme(o), l = e.getLanguage(i);
        return new Oe(St(n, l, s, a, r).stateStack, l.name, s.name);
    }
    function hc(t, e, n, r, i) {
        const o = St(t, e, n, r, i), s = new Oe(St(t, e, n, r, i).stateStack, e.name, n.name);
        return Ct(o.tokens, s), o.tokens;
    }
    function St(t, e, n, r, i) {
        const o = Et(n, i), { tokenizeMaxLineLength: s = 0, tokenizeTimeLimit: a = 500 } = i, l = kt(t);
        let c = i.grammarState ? tc(i.grammarState, n.name) ?? tn : i.grammarContextCode != null ? St(i.grammarContextCode, e, n, r, {
            ...i,
            grammarState: void 0,
            grammarContextCode: void 0
        }).stateStack : tn, d = [];
        const _ = [];
        for(let p = 0, f = l.length; p < f; p++){
            const [h, E] = l[p];
            if (h === "") {
                d = [], _.push([]);
                continue;
            }
            if (s > 0 && h.length >= s) {
                d = [], _.push([
                    {
                        content: h,
                        offset: E,
                        color: "",
                        fontStyle: 0
                    }
                ]);
                continue;
            }
            let g, w, m;
            i.includeExplanation && (g = e.tokenizeLine(h, c), w = g.tokens, m = 0);
            const y = e.tokenizeLine2(h, c, a), v = y.tokens.length / 2;
            for(let R = 0; R < v; R++){
                const O = y.tokens[2 * R], V = R + 1 < v ? y.tokens[2 * R + 2] : h.length;
                if (O === V) continue;
                const ee = y.tokens[2 * R + 1], oe = me(r[ke.getForeground(ee)], o), ue = ke.getFontStyle(ee), de = {
                    content: h.substring(O, V),
                    offset: E + O,
                    color: oe,
                    fontStyle: ue
                };
                if (i.includeExplanation) {
                    const we = [];
                    if (i.includeExplanation !== "scopeName") for (const q of n.settings){
                        let Y;
                        switch(typeof q.scope){
                            case "string":
                                Y = q.scope.split(/,/).map((Ee)=>Ee.trim());
                                break;
                            case "object":
                                Y = q.scope;
                                break;
                            default:
                                continue;
                        }
                        we.push({
                            settings: q,
                            selectors: Y.map((Ee)=>Ee.split(/ /))
                        });
                    }
                    de.explanation = [];
                    let X = 0;
                    for(; O + X < V;){
                        const q = w[m], Y = h.substring(q.startIndex, q.endIndex);
                        X += Y.length, de.explanation.push({
                            content: Y,
                            scopes: i.includeExplanation === "scopeName" ? fc(q.scopes) : pc(we, q.scopes)
                        }), m += 1;
                    }
                }
                d.push(de);
            }
            _.push(d), d = [], c = y.ruleStack;
        }
        return {
            tokens: _,
            stateStack: c
        };
    }
    function fc(t) {
        return t.map((e)=>({
                scopeName: e
            }));
    }
    function pc(t, e) {
        const n = [];
        for(let r = 0, i = e.length; r < i; r++){
            const o = e[r];
            n[r] = {
                scopeName: o,
                themeMatches: gc(t, o, e.slice(0, r))
            };
        }
        return n;
    }
    function Jn(t, e) {
        return t === e || e.substring(0, t.length) === t && e[t.length] === ".";
    }
    function mc(t, e, n) {
        if (!Jn(t[t.length - 1], e)) return !1;
        let r = t.length - 2, i = n.length - 1;
        for(; r >= 0 && i >= 0;)Jn(t[r], n[i]) && (r -= 1), i -= 1;
        return r === -1;
    }
    function gc(t, e, n) {
        const r = [];
        for (const { selectors: i, settings: o } of t)for (const s of i)if (mc(s, e, n)) {
            r.push(o);
            break;
        }
        return r;
    }
    function Xr(t, e, n) {
        const r = Object.entries(n.themes).filter((l)=>l[1]).map((l)=>({
                color: l[0],
                theme: l[1]
            })), i = r.map((l)=>{
            const c = pn(t, e, {
                ...n,
                theme: l.theme
            }), d = qe(c), _ = typeof l.theme == "string" ? l.theme : l.theme.name;
            return {
                tokens: c,
                state: d,
                theme: _
            };
        }), o = bc(...i.map((l)=>l.tokens)), s = o[0].map((l, c)=>l.map((d, _)=>{
                const p = {
                    content: d.content,
                    variants: {},
                    offset: d.offset
                };
                return "includeExplanation" in n && n.includeExplanation && (p.explanation = d.explanation), o.forEach((f, h)=>{
                    const { content: E, explanation: g, offset: w, ...m } = f[c][_];
                    p.variants[r[h].color] = m;
                }), p;
            })), a = i[0].state ? new Oe(Object.fromEntries(i.map((l)=>[
                l.theme,
                l.state?.getInternalStack(l.theme)
            ])), i[0].state.lang) : void 0;
        return a && Ct(s, a), s;
    }
    function bc(...t) {
        const e = t.map(()=>[]), n = t.length;
        for(let r = 0; r < t[0].length; r++){
            const i = t.map((l)=>l[r]), o = e.map(()=>[]);
            e.forEach((l, c)=>l.push(o[c]));
            const s = i.map(()=>0), a = i.map((l)=>l[0]);
            for(; a.every((l)=>l);){
                const l = Math.min(...a.map((c)=>c.content.length));
                for(let c = 0; c < n; c++){
                    const d = a[c];
                    d.content.length === l ? (o[c].push(d), s[c] += 1, a[c] = i[c][s[c]]) : (o[c].push({
                        ...d,
                        content: d.content.slice(0, l)
                    }), a[c] = {
                        ...d,
                        content: d.content.slice(l),
                        offset: d.offset + l
                    });
                }
            }
        }
        return e;
    }
    function At(t, e, n) {
        let r, i, o, s, a, l;
        if ("themes" in n) {
            const { defaultColor: c = "light", cssVariablePrefix: d = "--shiki-" } = n, _ = Object.entries(n.themes).filter((g)=>g[1]).map((g)=>({
                    color: g[0],
                    theme: g[1]
                })).sort((g, w)=>g.color === c ? -1 : w.color === c ? 1 : 0);
            if (_.length === 0) throw new ce("`themes` option must not be empty");
            const p = Xr(t, e, n);
            if (l = qe(p), c && !_.find((g)=>g.color === c)) throw new ce(`\`themes\` option must contain the defaultColor key \`${c}\``);
            const f = _.map((g)=>t.getTheme(g.theme)), h = _.map((g)=>g.color);
            o = p.map((g)=>g.map((w)=>yc(w, h, d, c))), l && Ct(o, l);
            const E = _.map((g)=>Et(g.theme, n));
            i = _.map((g, w)=>(w === 0 && c ? "" : `${d + g.color}:`) + (me(f[w].fg, E[w]) || "inherit")).join(";"), r = _.map((g, w)=>(w === 0 && c ? "" : `${d + g.color}-bg:`) + (me(f[w].bg, E[w]) || "inherit")).join(";"), s = `shiki-themes ${f.map((g)=>g.name).join(" ")}`, a = c ? void 0 : [
                i,
                r
            ].join(";");
        } else if ("theme" in n) {
            const c = Et(n.theme, n);
            o = pn(t, e, n);
            const d = t.getTheme(n.theme);
            r = me(d.bg, c), i = me(d.fg, c), s = d.name, l = qe(o);
        } else throw new ce("Invalid options, either `theme` or `themes` must be provided");
        return {
            tokens: o,
            fg: i,
            bg: r,
            themeName: s,
            rootStyle: a,
            grammarState: l
        };
    }
    function yc(t, e, n, r) {
        const i = {
            content: t.content,
            explanation: t.explanation,
            offset: t.offset
        }, o = e.map((l)=>zr(t.variants[l])), s = new Set(o.flatMap((l)=>Object.keys(l))), a = {};
        return o.forEach((l, c)=>{
            for (const d of s){
                const _ = l[d] || "inherit";
                if (c === 0 && r) a[d] = _;
                else {
                    const p = d === "color" ? "" : d === "background-color" ? "-bg" : `-${d}`, f = n + e[c] + (d === "color" ? "" : p);
                    a[f] = _;
                }
            }
        }), i.htmlStyle = a, i;
    }
    function Tt(t, e, n, r = {
        meta: {},
        options: n,
        codeToHast: (i, o)=>Tt(t, i, o),
        codeToTokens: (i, o)=>At(t, i, o)
    }) {
        let i = e;
        for (const f of vt(n))i = f.preprocess?.call(r, i, n) || i;
        let { tokens: o, fg: s, bg: a, themeName: l, rootStyle: c, grammarState: d } = At(t, i, n);
        const { mergeWhitespaces: _ = !0 } = n;
        _ === !0 ? o = Ec(o) : _ === "never" && (o = vc(o));
        const p = {
            ...r,
            get source () {
                return i;
            }
        };
        for (const f of vt(n))o = f.tokens?.call(p, o) || o;
        return wc(o, {
            ...n,
            fg: s,
            bg: a,
            themeName: l,
            rootStyle: c
        }, p, d);
    }
    function wc(t, e, n, r = qe(t)) {
        const i = vt(e), o = [], s = {
            type: "root",
            children: []
        }, { structure: a = "classic", tabindex: l = "0" } = e;
        let c = {
            type: "element",
            tagName: "pre",
            properties: {
                class: `shiki ${e.themeName || ""}`,
                style: e.rootStyle || `background-color:${e.bg};color:${e.fg}`,
                ...l !== !1 && l != null ? {
                    tabindex: l.toString()
                } : {},
                ...Object.fromEntries(Array.from(Object.entries(e.meta || {})).filter(([h])=>!h.startsWith("_")))
            },
            children: []
        }, d = {
            type: "element",
            tagName: "code",
            properties: {},
            children: o
        };
        const _ = [], p = {
            ...n,
            structure: a,
            addClassToHast: Wr,
            get source () {
                return n.source;
            },
            get tokens () {
                return t;
            },
            get options () {
                return e;
            },
            get root () {
                return s;
            },
            get pre () {
                return c;
            },
            get code () {
                return d;
            },
            get lines () {
                return _;
            }
        };
        if (t.forEach((h, E)=>{
            E && (a === "inline" ? s.children.push({
                type: "element",
                tagName: "br",
                properties: {},
                children: []
            }) : a === "classic" && o.push({
                type: "text",
                value: `
`
            }));
            let g = {
                type: "element",
                tagName: "span",
                properties: {
                    class: "line"
                },
                children: []
            }, w = 0;
            for (const m of h){
                let y = {
                    type: "element",
                    tagName: "span",
                    properties: {
                        ...m.htmlAttrs
                    },
                    children: [
                        {
                            type: "text",
                            value: m.content
                        }
                    ]
                };
                m.htmlStyle;
                const v = Zl(m.htmlStyle || zr(m));
                v && (y.properties.style = v);
                for (const R of i)y = R?.span?.call(p, y, E + 1, w, g, m) || y;
                a === "inline" ? s.children.push(y) : a === "classic" && g.children.push(y), w += m.content.length;
            }
            if (a === "classic") {
                for (const m of i)g = m?.line?.call(p, g, E + 1) || g;
                _.push(g), o.push(g);
            }
        }), a === "classic") {
            for (const h of i)d = h?.code?.call(p, d) || d;
            c.children.push(d);
            for (const h of i)c = h?.pre?.call(p, c) || c;
            s.children.push(c);
        }
        let f = s;
        for (const h of i)f = h?.root?.call(p, f) || f;
        return r && Ct(f, r), f;
    }
    function Ec(t) {
        return t.map((e)=>{
            const n = [];
            let r = "", i = 0;
            return e.forEach((o, s)=>{
                const l = !(o.fontStyle && o.fontStyle & ae.Underline);
                l && o.content.match(/^\s+$/) && e[s + 1] ? (i || (i = o.offset), r += o.content) : r ? (l ? n.push({
                    ...o,
                    offset: i,
                    content: r + o.content
                }) : n.push({
                    content: r,
                    offset: i
                }, o), i = 0, r = "") : n.push(o);
            }), n;
        });
    }
    function vc(t) {
        return t.map((e)=>e.flatMap((n)=>{
                if (n.content.match(/^\s+$/)) return n;
                const r = n.content.match(/^(\s*)(.*?)(\s*)$/);
                if (!r) return n;
                const [, i, o, s] = r;
                if (!i && !s) return n;
                const a = [
                    {
                        ...n,
                        offset: n.offset + i.length,
                        content: o
                    }
                ];
                return i && a.unshift({
                    content: i,
                    offset: n.offset
                }), s && a.push({
                    content: s,
                    offset: n.offset + i.length + o.length
                }), a;
            }));
    }
    function Sc(t, e, n) {
        const r = {
            meta: {},
            options: n,
            codeToHast: (o, s)=>Tt(t, o, s),
            codeToTokens: (o, s)=>At(t, o, s)
        };
        let i = zl(Tt(t, e, n, r));
        for (const o of vt(n))i = o.postprocess?.call(r, i, n) || i;
        return i;
    }
    const Xn = {
        light: "#333333",
        dark: "#bbbbbb"
    }, Yn = {
        light: "#fffffe",
        dark: "#1e1e1e"
    }, Qn = "__shiki_resolved";
    function mn(t) {
        if (t?.[Qn]) return t;
        const e = {
            ...t
        };
        e.tokenColors && !e.settings && (e.settings = e.tokenColors, delete e.tokenColors), e.type ||= "dark", e.colorReplacements = {
            ...e.colorReplacements
        }, e.settings ||= [];
        let { bg: n, fg: r } = e;
        if (!n || !r) {
            const a = e.settings ? e.settings.find((l)=>!l.name && !l.scope) : void 0;
            a?.settings?.foreground && (r = a.settings.foreground), a?.settings?.background && (n = a.settings.background), !r && e?.colors?.["editor.foreground"] && (r = e.colors["editor.foreground"]), !n && e?.colors?.["editor.background"] && (n = e.colors["editor.background"]), r || (r = e.type === "light" ? Xn.light : Xn.dark), n || (n = e.type === "light" ? Yn.light : Yn.dark), e.fg = r, e.bg = n;
        }
        e.settings[0] && e.settings[0].settings && !e.settings[0].scope || e.settings.unshift({
            settings: {
                foreground: e.fg,
                background: e.bg
            }
        });
        let i = 0;
        const o = new Map;
        function s(a) {
            if (o.has(a)) return o.get(a);
            i += 1;
            const l = `#${i.toString(16).padStart(8, "0").toLowerCase()}`;
            return e.colorReplacements?.[`#${l}`] ? s(a) : (o.set(a, l), l);
        }
        e.settings = e.settings.map((a)=>{
            const l = a.settings?.foreground && !a.settings.foreground.startsWith("#"), c = a.settings?.background && !a.settings.background.startsWith("#");
            if (!l && !c) return a;
            const d = {
                ...a,
                settings: {
                    ...a.settings
                }
            };
            if (l) {
                const _ = s(a.settings.foreground);
                e.colorReplacements[_] = a.settings.foreground, d.settings.foreground = _;
            }
            if (c) {
                const _ = s(a.settings.background);
                e.colorReplacements[_] = a.settings.background, d.settings.background = _;
            }
            return d;
        });
        for (const a of Object.keys(e.colors || {}))if ((a === "editor.foreground" || a === "editor.background" || a.startsWith("terminal.ansi")) && !e.colors[a]?.startsWith("#")) {
            const l = s(e.colors[a]);
            e.colorReplacements[l] = e.colors[a], e.colors[a] = l;
        }
        return Object.defineProperty(e, Qn, {
            enumerable: !1,
            writable: !1,
            value: !0
        }), e;
    }
    async function Yr(t) {
        return Array.from(new Set((await Promise.all(t.filter((e)=>!Hr(e)).map(async (e)=>await qr(e).then((n)=>Array.isArray(n) ? n : [
                    n
                ])))).flat()));
    }
    async function Qr(t) {
        return (await Promise.all(t.map(async (n)=>Fr(n) ? null : mn(await qr(n))))).filter((n)=>!!n);
    }
    class Ac extends xa {
        constructor(e, n, r, i = {}){
            super(e), this._resolver = e, this._themes = n, this._langs = r, this._alias = i, this._themes.map((o)=>this.loadTheme(o)), this.loadLanguages(this._langs);
        }
        _resolvedThemes = new Map;
        _resolvedGrammars = new Map;
        _langMap = new Map;
        _langGraph = new Map;
        _textmateThemeCache = new WeakMap;
        _loadedThemesCache = null;
        _loadedLanguagesCache = null;
        getTheme(e) {
            return typeof e == "string" ? this._resolvedThemes.get(e) : this.loadTheme(e);
        }
        loadTheme(e) {
            const n = mn(e);
            return n.name && (this._resolvedThemes.set(n.name, n), this._loadedThemesCache = null), n;
        }
        getLoadedThemes() {
            return this._loadedThemesCache || (this._loadedThemesCache = [
                ...this._resolvedThemes.keys()
            ]), this._loadedThemesCache;
        }
        setTheme(e) {
            let n = this._textmateThemeCache.get(e);
            n || (n = mt.createFromRawTheme(e), this._textmateThemeCache.set(e, n)), this._syncRegistry.setTheme(n);
        }
        getGrammar(e) {
            if (this._alias[e]) {
                const n = new Set([
                    e
                ]);
                for(; this._alias[e];){
                    if (e = this._alias[e], n.has(e)) throw new W(`Circular alias \`${Array.from(n).join(" -> ")} -> ${e}\``);
                    n.add(e);
                }
            }
            return this._resolvedGrammars.get(e);
        }
        loadLanguage(e) {
            if (this.getGrammar(e.name)) return;
            const n = new Set([
                ...this._langMap.values()
            ].filter((o)=>o.embeddedLangsLazy?.includes(e.name)));
            this._resolver.addLanguage(e);
            const r = {
                balancedBracketSelectors: e.balancedBracketSelectors || [
                    "*"
                ],
                unbalancedBracketSelectors: e.unbalancedBracketSelectors || []
            };
            this._syncRegistry._rawGrammars.set(e.scopeName, e);
            const i = this.loadGrammarWithConfiguration(e.scopeName, 1, r);
            if (i.name = e.name, this._resolvedGrammars.set(e.name, i), e.aliases && e.aliases.forEach((o)=>{
                this._alias[o] = e.name;
            }), this._loadedLanguagesCache = null, n.size) for (const o of n)this._resolvedGrammars.delete(o.name), this._loadedLanguagesCache = null, this._syncRegistry?._injectionGrammars?.delete(o.scopeName), this._syncRegistry?._grammars?.delete(o.scopeName), this.loadLanguage(this._langMap.get(o.name));
        }
        dispose() {
            super.dispose(), this._resolvedThemes.clear(), this._resolvedGrammars.clear(), this._langMap.clear(), this._langGraph.clear(), this._loadedThemesCache = null;
        }
        loadLanguages(e) {
            for (const i of e)this.resolveEmbeddedLanguages(i);
            const n = Array.from(this._langGraph.entries()), r = n.filter(([i, o])=>!o);
            if (r.length) {
                const i = n.filter(([o, s])=>s && s.embeddedLangs?.some((a)=>r.map(([l])=>l).includes(a))).filter((o)=>!r.includes(o));
                throw new W(`Missing languages ${r.map(([o])=>`\`${o}\``).join(", ")}, required by ${i.map(([o])=>`\`${o}\``).join(", ")}`);
            }
            for (const [i, o] of n)this._resolver.addLanguage(o);
            for (const [i, o] of n)this.loadLanguage(o);
        }
        getLoadedLanguages() {
            return this._loadedLanguagesCache || (this._loadedLanguagesCache = [
                ...new Set([
                    ...this._resolvedGrammars.keys(),
                    ...Object.keys(this._alias)
                ])
            ]), this._loadedLanguagesCache;
        }
        resolveEmbeddedLanguages(e) {
            if (this._langMap.set(e.name, e), this._langGraph.set(e.name, e), e.embeddedLangs) for (const n of e.embeddedLangs)this._langGraph.set(n, this._langMap.get(n));
        }
    }
    class Tc {
        _langs = new Map;
        _scopeToLang = new Map;
        _injections = new Map;
        _onigLib;
        constructor(e, n){
            this._onigLib = {
                createOnigScanner: (r)=>e.createScanner(r),
                createOnigString: (r)=>e.createString(r)
            }, n.forEach((r)=>this.addLanguage(r));
        }
        get onigLib() {
            return this._onigLib;
        }
        getLangRegistration(e) {
            return this._langs.get(e);
        }
        loadGrammar(e) {
            return this._scopeToLang.get(e);
        }
        addLanguage(e) {
            this._langs.set(e.name, e), e.aliases && e.aliases.forEach((n)=>{
                this._langs.set(n, e);
            }), this._scopeToLang.set(e.scopeName, e), e.injectTo && e.injectTo.forEach((n)=>{
                this._injections.get(n) || this._injections.set(n, []), this._injections.get(n).push(e.scopeName);
            });
        }
        getInjections(e) {
            const n = e.split(".");
            let r = [];
            for(let i = 1; i <= n.length; i++){
                const o = n.slice(0, i).join(".");
                r = [
                    ...r,
                    ...this._injections.get(o) || []
                ];
            }
            return r;
        }
    }
    let De = 0;
    function Rc(t) {
        De += 1, t.warnings !== !1 && De >= 10 && De % 10 === 0 && console.warn(`[Shiki] ${De} instances have been created. Shiki is supposed to be used as a singleton, consider refactoring your code to cache your highlighter instance; Or call \`highlighter.dispose()\` to release unused instances.`);
        let e = !1;
        if (!t.engine) throw new W("`engine` option is required for synchronous mode");
        const n = (t.langs || []).flat(1), r = (t.themes || []).flat(1).map(mn), i = new Tc(t.engine, n), o = new Ac(i, r, n, t.langAlias);
        let s;
        function a(m) {
            g();
            const y = o.getGrammar(typeof m == "string" ? m : m.name);
            if (!y) throw new W(`Language \`${m}\` not found, you may need to load it first`);
            return y;
        }
        function l(m) {
            if (m === "none") return {
                bg: "",
                fg: "",
                name: "none",
                settings: [],
                type: "dark"
            };
            g();
            const y = o.getTheme(m);
            if (!y) throw new W(`Theme \`${m}\` not found, you may need to load it first`);
            return y;
        }
        function c(m) {
            g();
            const y = l(m);
            s !== m && (o.setTheme(y), s = m);
            const v = o.getColorMap();
            return {
                theme: y,
                colorMap: v
            };
        }
        function d() {
            return g(), o.getLoadedThemes();
        }
        function _() {
            return g(), o.getLoadedLanguages();
        }
        function p(...m) {
            g(), o.loadLanguages(m.flat(1));
        }
        async function f(...m) {
            return p(await Yr(m));
        }
        function h(...m) {
            g();
            for (const y of m.flat(1))o.loadTheme(y);
        }
        async function E(...m) {
            return g(), h(await Qr(m));
        }
        function g() {
            if (e) throw new W("Shiki instance has been disposed");
        }
        function w() {
            e || (e = !0, o.dispose(), De -= 1);
        }
        return {
            setTheme: c,
            getTheme: l,
            getLanguage: a,
            getLoadedThemes: d,
            getLoadedLanguages: _,
            loadLanguage: f,
            loadLanguageSync: p,
            loadTheme: E,
            loadThemeSync: h,
            dispose: w,
            [Symbol.dispose]: w
        };
    }
    async function Lc(t = {}) {
        t.loadWasm;
        const [e, n, r] = await Promise.all([
            Qr(t.themes || []),
            Yr(t.langs || []),
            t.engine || fr(t.loadWasm || zs())
        ]);
        return Rc({
            ...t,
            themes: e,
            langs: n,
            engine: r
        });
    }
    async function Pc(t = {}) {
        const e = await Lc(t);
        return {
            getLastGrammarState: (...n)=>_c(e, ...n),
            codeToTokensBase: (n, r)=>pn(e, n, r),
            codeToTokensWithThemes: (n, r)=>Xr(e, n, r),
            codeToTokens: (n, r)=>At(e, n, r),
            codeToHast: (n, r)=>Tt(e, n, r),
            codeToHtml: (n, r)=>Sc(e, n, r),
            ...e,
            getInternalContext: ()=>e
        };
    }
    function kc(t, e, n) {
        let r, i, o;
        {
            const a = t;
            r = a.langs, i = a.themes, o = a.engine;
        }
        async function s(a) {
            function l(f) {
                if (typeof f == "string") {
                    if (Hr(f)) return [];
                    const h = r[f];
                    if (!h) throw new ce(`Language \`${f}\` is not included in this bundle. You may want to load it from external source.`);
                    return h;
                }
                return f;
            }
            function c(f) {
                if (Fr(f)) return "none";
                if (typeof f == "string") {
                    const h = i[f];
                    if (!h) throw new ce(`Theme \`${f}\` is not included in this bundle. You may want to load it from external source.`);
                    return h;
                }
                return f;
            }
            const d = (a.themes ?? []).map((f)=>c(f)), _ = (a.langs ?? []).map((f)=>l(f)), p = await Pc({
                engine: a.engine ?? o(),
                ...a,
                themes: d,
                langs: _
            });
            return {
                ...p,
                loadLanguage (...f) {
                    return p.loadLanguage(...f.map(l));
                },
                loadTheme (...f) {
                    return p.loadTheme(...f.map(c));
                }
            };
        }
        return s;
    }
    function Cc(t) {
        let e;
        async function n(r = {}) {
            if (e) {
                const i = await e;
                return await Promise.all([
                    i.loadTheme(...r.themes || []),
                    i.loadLanguage(...r.langs || [])
                ]), i;
            } else return e = t({
                ...r,
                themes: r.themes || [],
                langs: r.langs || []
            }), e;
        }
        return n;
    }
    function Ic(t) {
        const e = Cc(t);
        return {
            getSingletonHighlighter (n) {
                return e(n);
            },
            async codeToHtml (n, r) {
                return (await e({
                    langs: [
                        r.lang
                    ],
                    themes: "theme" in r ? [
                        r.theme
                    ] : Object.values(r.themes)
                })).codeToHtml(n, r);
            },
            async codeToHast (n, r) {
                return (await e({
                    langs: [
                        r.lang
                    ],
                    themes: "theme" in r ? [
                        r.theme
                    ] : Object.values(r.themes)
                })).codeToHast(n, r);
            },
            async codeToTokens (n, r) {
                return (await e({
                    langs: [
                        r.lang
                    ],
                    themes: "theme" in r ? [
                        r.theme
                    ] : Object.values(r.themes)
                })).codeToTokens(n, r);
            },
            async codeToTokensBase (n, r) {
                return (await e({
                    langs: [
                        r.lang
                    ],
                    themes: [
                        r.theme
                    ]
                })).codeToTokensBase(n, r);
            },
            async codeToTokensWithThemes (n, r) {
                return (await e({
                    langs: [
                        r.lang
                    ],
                    themes: Object.values(r.themes).filter(Boolean)
                })).codeToTokensWithThemes(n, r);
            },
            async getLastGrammarState (n, r) {
                return (await e({
                    langs: [
                        r.lang
                    ],
                    themes: [
                        r.theme
                    ]
                })).getLastGrammarState(n, r);
            }
        };
    }
    const Oc = kc({
        langs: Ls,
        themes: ks,
        engine: ()=>fr(u(()=>import("./wasm-CG6Dc4jp.js"), []))
    }), { codeToHtml: xc } = Ic(Oc);
    var Dc = N("<div class=html-viewer-container>");
    function Nc(t) {
        const [e, n] = te(""), r = (i)=>{
            let o = "", s = 0;
            const a = "  ";
            return i.split(/(<[^>]+>)/g).filter((c)=>c.trim()).forEach((c)=>{
                if (c.startsWith("</")) s = Math.max(0, s - 1), o += a.repeat(s) + c + `
`;
                else if (c.startsWith("<")) {
                    const d = c.endsWith("/>") || /^<(area|base|br|col|embed|hr|img|input|link|meta|param|source|track|wbr)/.test(c);
                    o += a.repeat(s) + c + `
`, d || s++;
                } else {
                    const d = c.trim();
                    d && (o += a.repeat(s) + d + `
`);
                }
            }), o.trimEnd();
        };
        return nr(async ()=>{
            const i = r(t.html);
            try {
                const o = await xc(i, {
                    lang: "html",
                    theme: "github-dark"
                });
                n(o);
            } catch (o) {
                console.error("Shiki highlighting error:", o), n(`<pre><code>${i}</code></pre>`);
            }
        }), (()=>{
            var i = Dc();
            return ie(()=>i.innerHTML = e()), i;
        })();
    }
    var Vc = N('<div class="border-b border-gray-200 bg-white px-6 py-4"><h2 class="mb-3 text-sm font-semibold text-gray-700">Parser Options</h2><div class="grid grid-cols-2 gap-3 md:grid-cols-4">'), Zn = N('<div class="json-viewer-container flex-1 overflow-auto rounded-lg border border-gray-300 bg-gray-900 p-4">'), Mc = N('<div class="flex-1 overflow-auto rounded-lg border border-gray-300 bg-gray-900">'), $c = N('<div class="markdown-preview flex-1 overflow-auto rounded-lg border border-gray-300 bg-white p-6">'), jc = N('<main class="flex h-full w-full flex-col bg-gray-50"><header class="border-b border-gray-200 bg-white px-6 py-4 shadow-sm"><div class="flex items-center justify-between"><div><h1 class="text-2xl font-bold text-gray-900">Markdown Parser Playground</h1><p class="text-sm text-gray-500">Test and explore markdown parsing with various options</p></div><button class="rounded-lg border border-gray-300 bg-white px-4 py-2 text-sm font-medium text-gray-700 hover:bg-gray-50"> Options</button></div></header><div class="flex flex-1 gap-4 overflow-hidden p-6"><div class="flex flex-1 flex-col"><div class="mb-2 flex items-center justify-between"><label for=input class="text-sm font-semibold text-gray-700">Input</label><button class="rounded px-2 py-1 text-xs text-gray-500 hover:bg-gray-100">Clear</button></div><textarea id=input class="flex-1 resize-none rounded-lg border border-gray-300 bg-white p-4 font-mono text-sm outline-none focus:border-blue-500 focus:ring-2 focus:ring-blue-200"placeholder="Enter markdown text here..."></textarea></div><div class="flex flex-1 flex-col"><div class="mb-2 flex items-center justify-between"><div class="flex items-center gap-3"><span class="text-sm font-semibold text-gray-700">Output</span><span class="rounded-full bg-blue-100 px-2.5 py-0.5 text-xs font-semibold text-blue-800">ms</span></div><button class="rounded px-2 py-1 text-xs text-gray-500 hover:bg-gray-100">Copy</button></div><div class="mb-3 flex gap-1"><button><svg class="h-3.5 w-3.5"fill=none stroke=currentColor viewBox="0 0 24 24"><path stroke-linecap=round stroke-linejoin=round stroke-width=2 d="M10 20l4-16m4 4l4 4-4 4M6 16l-4-4 4-4"></path></svg>AST</button><button><svg class="h-3.5 w-3.5"fill=none stroke=currentColor viewBox="0 0 24 24"><path stroke-linecap=round stroke-linejoin=round stroke-width=2 d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z"></path></svg>Frontmatter</button><button><svg class="h-3.5 w-3.5"fill=none stroke=currentColor viewBox="0 0 24 24"><path stroke-linecap=round stroke-linejoin=round stroke-width=2 d="M7 21h10a2 2 0 002-2V9.414a1 1 0 00-.293-.707l-5.414-5.414A1 1 0 0012.586 3H7a2 2 0 00-2 2v14a2 2 0 002 2z"></path></svg>HTML</button><button><svg class="h-3.5 w-3.5"fill=none stroke=currentColor viewBox="0 0 24 24"><path stroke-linecap=round stroke-linejoin=round stroke-width=2 d="M15 12a3 3 0 11-6 0 3 3 0 016 0z"></path><path stroke-linecap=round stroke-linejoin=round stroke-width=2 d="M2.458 12C3.732 7.943 7.523 5 12 5c4.478 0 8.268 2.943 9.542 7-1.274 4.057-5.064 7-9.542 7-4.477 0-8.268-2.943-9.542-7z"></path></svg>Preview'), Bc = N('<label class="flex cursor-pointer items-start gap-2"><input type=checkbox class="mt-0.5 h-4 w-4 rounded border-gray-300 text-blue-600 focus:ring-2 focus:ring-blue-500"><div class=flex-1><div class="text-sm font-medium text-gray-700">'), Gc = N('<div class="text-xs text-gray-500">'), Uc = N('<div class="flex h-full items-center justify-center text-gray-500"><div class=text-center><svg class="mx-auto h-12 w-12 text-gray-400"fill=none stroke=currentColor viewBox="0 0 24 24"><path stroke-linecap=round stroke-linejoin=round stroke-width=2 d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z"></path></svg><p class="mt-2 text-sm">No frontmatter found</p><p class="mt-1 text-xs text-gray-600">Add YAML frontmatter at the top of your markdown');
    const Hc = `---
title: Markdown Parser Playground
author: Demo User
date: 2024-01-15
tags: [markdown, parser, demo]
draft: false
---

# Markdown Parser Playground

Try editing this markdown text to see the AST output!

## Features

- **GitHub Flavored Markdown** (GFM)
- **Obsidian Flavored Markdown** (OFM)
- CJK text support
- Smart punctuation
- And more...

\`\`\`javascript
console.log('Hello, World!');
\`\`\`

| Feature | Supported |
|---------|-----------|
| Tables  | ✓         |
| Lists   | ✓         |

#tag [[wikilink]]
`, Fc = [
        {
            key: "github_flavored",
            label: "GitHub Flavored",
            description: "Enable GFM extensions"
        },
        {
            key: "gfm_extended_autolink",
            label: "GFM Autolink",
            description: "Extended autolink support"
        },
        {
            key: "obsidian_flavored",
            label: "Obsidian Flavored",
            description: "Enable OFM extensions"
        },
        {
            key: "jsx_like_component",
            label: "JSX Like Component",
            description: "Support JSX Like components"
        },
        {
            key: "cjk_autocorrect",
            label: "CJK Autocorrect",
            description: "Auto-correct CJK spacing"
        },
        {
            key: "smart_punctuation",
            label: "Smart Punctuation",
            description: "Convert quotes and dashes"
        },
        {
            key: "normalize_chinese_punctuation",
            label: "Normalize Chinese Punct",
            description: "Normalize Chinese punctuation"
        },
        {
            key: "cjk_friendly_delimiters",
            label: "CJK Friendly Delimiters",
            description: "CJK-friendly emphasis delimiters"
        }
    ];
    function Wc() {
        const [t, e] = te(Hc), [n, r] = te(null), [i, o] = te(""), [s, a] = te(null), [l, c] = te(), [d, _] = te(!1), [p, f] = te("ast"), [h, E] = te({
            github_flavored: !0,
            obsidian_flavored: !0,
            cjk_autocorrect: !0
        });
        let g;
        const w = (y)=>{
            E((v)=>({
                    ...v,
                    [y]: !v[y]
                }));
        };
        nr(()=>{
            const y = t(), v = h(), R = performance.now(), O = fs(y, v);
            c(Math.ceil((performance.now() - R) * 100) / 100), r(O.tree), o(O.toHtml());
            const V = O.frontmatter;
            if (V instanceof Map) {
                const ee = {};
                V.forEach((oe, ue)=>{
                    ee[ue] = oe;
                }), a(ee);
            } else a(V);
        });
        const m = (y, v)=>{
            if (!g || !y || !v) return;
            const O = t().split(`
`);
            let V = 0;
            for(let X = 0; X < y.line - 1; X++)V += O[X].length + 1;
            V += y.column - 1;
            let ee = 0;
            for(let X = 0; X < v.line - 1; X++)ee += O[X].length + 1;
            ee += v.column - 1, g.focus(), g.setSelectionRange(V, ee);
            const oe = parseInt(getComputedStyle(g).lineHeight) || 20, ue = (y.line - 1) * oe, de = g.clientHeight, we = ue - de / 2 + oe / 2;
            g.scrollTo({
                top: Math.max(0, we),
                behavior: "smooth"
            });
        };
        return (()=>{
            var y = jc(), v = y.firstChild, R = v.firstChild, O = R.firstChild, V = O.nextSibling, ee = V.firstChild, oe = v.nextSibling, ue = oe.firstChild, de = ue.firstChild, we = de.firstChild, X = we.nextSibling, q = de.nextSibling, Y = ue.nextSibling, Ee = Y.firstChild, gn = Ee.firstChild, Zr = gn.firstChild, bn = Zr.nextSibling, ei = bn.firstChild, ti = gn.nextSibling, ni = Ee.nextSibling, It = ni.firstChild, Ot = It.nextSibling, xt = Ot.nextSibling, yn = xt.nextSibling;
            V.$$click = ()=>_(!d()), L(V, ()=>d() ? "Hide" : "Show", ee), L(y, I(U, {
                get when () {
                    return d();
                },
                get children () {
                    var P = Vc(), Qe = P.firstChild, Ze = Qe.nextSibling;
                    return L(Ze, I(lr, {
                        each: Fc,
                        children: (_e)=>(()=>{
                                var xe = Bc(), Dt = xe.firstChild, En = Dt.nextSibling, ri = En.firstChild;
                                return Dt.addEventListener("change", ()=>w(_e.key)), L(ri, ()=>_e.label), L(En, (()=>{
                                    var ii = Me(()=>!!_e.description);
                                    return ()=>ii() && (()=>{
                                            var vn = Gc();
                                            return L(vn, ()=>_e.description), vn;
                                        })();
                                })(), null), ie(()=>Dt.checked = !!h()[_e.key]), xe;
                            })()
                    })), P;
                }
            }), oe), X.$$click = ()=>e(""), yi(q, "input", (P)=>e(P.currentTarget.value));
            var wn = g;
            return typeof wn == "function" ? wi(wn, q) : g = q, L(bn, l, ei), ti.$$click = ()=>{
                const P = p() === "ast" ? JSON.stringify(n(), null, 2) : i();
                navigator.clipboard.writeText(P);
            }, It.$$click = ()=>f("ast"), Ot.$$click = ()=>f("frontmatter"), xt.$$click = ()=>f("html"), yn.$$click = ()=>f("preview"), L(Y, I(U, {
                get when () {
                    return p() === "ast";
                },
                get children () {
                    var P = Zn();
                    return L(P, I(U, {
                        get when () {
                            return n();
                        },
                        get children () {
                            return I(Kt, {
                                get data () {
                                    return n();
                                },
                                onNodeClick: m
                            });
                        }
                    })), P;
                }
            }), null), L(Y, I(U, {
                get when () {
                    return p() === "frontmatter";
                },
                get children () {
                    var P = Zn();
                    return L(P, I(U, {
                        get when () {
                            return Me(()=>!!(s() && typeof s() == "object"))() && Object.keys(s()).length > 0;
                        },
                        get fallback () {
                            return Uc();
                        },
                        get children () {
                            return I(Kt, {
                                get data () {
                                    return s();
                                }
                            });
                        }
                    })), P;
                }
            }), null), L(Y, I(U, {
                get when () {
                    return p() === "html";
                },
                get children () {
                    var P = Mc();
                    return L(P, I(Nc, {
                        get html () {
                            return i();
                        }
                    })), P;
                }
            }), null), L(Y, I(U, {
                get when () {
                    return p() === "preview";
                },
                get children () {
                    var P = $c();
                    return ie(()=>P.innerHTML = i()), P;
                }
            }), null), ie((P)=>{
                var Qe = `tab-button ${p() === "ast" ? "tab-button-active" : ""}`, Ze = `tab-button ${p() === "frontmatter" ? "tab-button-active" : ""}`, _e = `tab-button ${p() === "html" ? "tab-button-active" : ""}`, xe = `tab-button ${p() === "preview" ? "tab-button-active" : ""}`;
                return Qe !== P.e && Ne(It, P.e = Qe), Ze !== P.t && Ne(Ot, P.t = Ze), _e !== P.a && Ne(xt, P.a = _e), xe !== P.o && Ne(yn, P.o = xe), P;
            }, {
                e: void 0,
                t: void 0,
                a: void 0,
                o: void 0
            }), ie(()=>q.value = t()), y;
        })();
    }
    cr([
        "click"
    ]);
    const qc = document.getElementById("root");
    bi(()=>I(Wc, {}), qc);
})();

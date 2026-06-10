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
    const ti = !1, ni = (t, e)=>t === e, ri = Symbol("solid-track"), lt = {
        equals: ni
    };
    let Qn = rr;
    const fe = 1, ct = 2, Zn = {
        owned: null,
        cleanups: null,
        context: null,
        owner: null
    };
    var M = null;
    let xt = null, ii = null, D = null, H = null, le = null, St = 0;
    function rt(t, e) {
        const n = D, r = M, i = t.length === 0, o = e === void 0 ? r : e, s = i ? Zn : {
            owned: null,
            cleanups: null,
            context: o ? o.context : null,
            owner: o
        }, a = i ? t : ()=>t(()=>ye(()=>Ne(s)));
        M = s, D = null;
        try {
            return Ue(a, !0);
        } finally{
            D = n, M = r;
        }
    }
    function ee(t, e) {
        e = e ? Object.assign({}, lt, e) : lt;
        const n = {
            value: t,
            observers: null,
            observerSlots: null,
            comparator: e.equals || void 0
        }, r = (i)=>(typeof i == "function" && (i = i(n.value)), nr(n, i));
        return [
            tr.bind(n),
            r
        ];
    }
    function re(t, e, n) {
        const r = rn(t, e, !1, fe);
        Ge(r);
    }
    function er(t, e, n) {
        Qn = li;
        const r = rn(t, e, !1, fe);
        r.user = !0, le ? le.push(r) : Ge(r);
    }
    function ie(t, e, n) {
        n = n ? Object.assign({}, lt, n) : lt;
        const r = rn(t, e, !0, 0);
        return r.observers = null, r.observerSlots = null, r.comparator = n.equals || void 0, Ge(r), tr.bind(r);
    }
    function ye(t) {
        if (D === null) return t();
        const e = D;
        D = null;
        try {
            return t();
        } finally{
            D = e;
        }
    }
    function oi(t) {
        return M === null || (M.cleanups === null ? M.cleanups = [
            t
        ] : M.cleanups.push(t)), t;
    }
    function tr() {
        if (this.sources && this.state) if (this.state === fe) Ge(this);
        else {
            const t = H;
            H = null, Ue(()=>dt(this), !1), H = t;
        }
        if (D) {
            const t = this.observers ? this.observers.length : 0;
            D.sources ? (D.sources.push(this), D.sourceSlots.push(t)) : (D.sources = [
                this
            ], D.sourceSlots = [
                t
            ]), this.observers ? (this.observers.push(D), this.observerSlots.push(D.sources.length - 1)) : (this.observers = [
                D
            ], this.observerSlots = [
                D.sources.length - 1
            ]);
        }
        return this.value;
    }
    function nr(t, e, n) {
        let r = t.value;
        return (!t.comparator || !t.comparator(r, e)) && (t.value = e, t.observers && t.observers.length && Ue(()=>{
            for(let i = 0; i < t.observers.length; i += 1){
                const o = t.observers[i], s = xt && xt.running;
                s && xt.disposed.has(o), (s ? !o.tState : !o.state) && (o.pure ? H.push(o) : le.push(o), o.observers && ir(o)), s || (o.state = fe);
            }
            if (H.length > 1e6) throw H = [], new Error;
        }, !1)), e;
    }
    function Ge(t) {
        if (!t.fn) return;
        Ne(t);
        const e = St;
        si(t, t.value, e);
    }
    function si(t, e, n) {
        let r;
        const i = M, o = D;
        D = M = t;
        try {
            r = t.fn(e);
        } catch (s) {
            return t.pure && (t.state = fe, t.owned && t.owned.forEach(Ne), t.owned = null), t.updatedAt = n + 1, or(s);
        } finally{
            D = o, M = i;
        }
        (!t.updatedAt || t.updatedAt <= n) && (t.updatedAt != null && "observers" in t ? nr(t, r) : t.value = r, t.updatedAt = n);
    }
    function rn(t, e, n, r = fe, i) {
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
        return M === null || M !== Zn && (M.owned ? M.owned.push(o) : M.owned = [
            o
        ]), o;
    }
    function ut(t) {
        if (t.state === 0) return;
        if (t.state === ct) return dt(t);
        if (t.suspense && ye(t.suspense.inFallback)) return t.suspense.effects.push(t);
        const e = [
            t
        ];
        for(; (t = t.owner) && (!t.updatedAt || t.updatedAt < St);)t.state && e.push(t);
        for(let n = e.length - 1; n >= 0; n--)if (t = e[n], t.state === fe) Ge(t);
        else if (t.state === ct) {
            const r = H;
            H = null, Ue(()=>dt(t, e[0]), !1), H = r;
        }
    }
    function Ue(t, e) {
        if (H) return t();
        let n = !1;
        e || (H = []), le ? n = !0 : le = [], St++;
        try {
            const r = t();
            return ai(n), r;
        } catch (r) {
            n || (le = null), H = null, or(r);
        }
    }
    function ai(t) {
        if (H && (rr(H), H = null), t) return;
        const e = le;
        le = null, e.length && Ue(()=>Qn(e), !1);
    }
    function rr(t) {
        for(let e = 0; e < t.length; e++)ut(t[e]);
    }
    function li(t) {
        let e, n = 0;
        for(e = 0; e < t.length; e++){
            const r = t[e];
            r.user ? t[n++] = r : ut(r);
        }
        for(e = 0; e < n; e++)ut(t[e]);
    }
    function dt(t, e) {
        t.state = 0;
        for(let n = 0; n < t.sources.length; n += 1){
            const r = t.sources[n];
            if (r.sources) {
                const i = r.state;
                i === fe ? r !== e && (!r.updatedAt || r.updatedAt < St) && ut(r) : i === ct && dt(r, e);
            }
        }
    }
    function ir(t) {
        for(let e = 0; e < t.observers.length; e += 1){
            const n = t.observers[e];
            n.state || (n.state = ct, n.pure ? H.push(n) : le.push(n), n.observers && ir(n));
        }
    }
    function Ne(t) {
        let e;
        if (t.sources) for(; t.sources.length;){
            const n = t.sources.pop(), r = t.sourceSlots.pop(), i = n.observers;
            if (i && i.length) {
                const o = i.pop(), s = n.observerSlots.pop();
                r < i.length && (o.sourceSlots[s] = r, i[r] = o, n.observerSlots[r] = s);
            }
        }
        if (t.tOwned) {
            for(e = t.tOwned.length - 1; e >= 0; e--)Ne(t.tOwned[e]);
            delete t.tOwned;
        }
        if (t.owned) {
            for(e = t.owned.length - 1; e >= 0; e--)Ne(t.owned[e]);
            t.owned = null;
        }
        if (t.cleanups) {
            for(e = t.cleanups.length - 1; e >= 0; e--)t.cleanups[e]();
            t.cleanups = null;
        }
        t.state = 0;
    }
    function ci(t) {
        return t instanceof Error ? t : new Error(typeof t == "string" ? t : "Unknown error", {
            cause: t
        });
    }
    function or(t, e = M) {
        throw ci(t);
    }
    const ui = Symbol("fallback");
    function wn(t) {
        for(let e = 0; e < t.length; e++)t[e]();
    }
    function di(t, e, n = {}) {
        let r = [], i = [], o = [], s = 0, a = e.length > 1 ? [] : null;
        return oi(()=>wn(o)), ()=>{
            let l = t() || [], u = l.length, h, d;
            return l[ri], ye(()=>{
                let f, m, w, g, E, p, b, v, T;
                if (u === 0) s !== 0 && (wn(o), o = [], r = [], i = [], s = 0, a && (a = [])), n.fallback && (r = [
                    ui
                ], i[0] = rt((O)=>(o[0] = O, n.fallback())), s = 1);
                else if (s === 0) {
                    for(i = new Array(u), d = 0; d < u; d++)r[d] = l[d], i[d] = rt(_);
                    s = u;
                } else {
                    for(w = new Array(u), g = new Array(u), a && (E = new Array(u)), p = 0, b = Math.min(s, u); p < b && r[p] === l[p]; p++);
                    for(b = s - 1, v = u - 1; b >= p && v >= p && r[b] === l[v]; b--, v--)w[v] = i[b], g[v] = o[b], a && (E[v] = a[b]);
                    for(f = new Map, m = new Array(v + 1), d = v; d >= p; d--)T = l[d], h = f.get(T), m[d] = h === void 0 ? -1 : h, f.set(T, d);
                    for(h = p; h <= b; h++)T = r[h], d = f.get(T), d !== void 0 && d !== -1 ? (w[d] = i[h], g[d] = o[h], a && (E[d] = a[h]), d = m[d], f.set(T, d)) : o[h]();
                    for(d = p; d < u; d++)d in w ? (i[d] = w[d], o[d] = g[d], a && (a[d] = E[d], a[d](d))) : i[d] = rt(_);
                    i = i.slice(0, s = u), r = l.slice(0);
                }
                return i;
            });
            function _(f) {
                if (o[d] = f, a) {
                    const [m, w] = ee(d);
                    return a[d] = w, e(l[d], m);
                }
                return e(l[d]);
            }
        };
    }
    function I(t, e) {
        return ye(()=>t(e || {}));
    }
    const hi = (t)=>`Stale read from <${t}>.`;
    function sr(t) {
        const e = "fallback" in t && {
            fallback: ()=>t.fallback
        };
        return ie(di(()=>t.each, t.children, e || void 0));
    }
    function G(t) {
        const e = t.keyed, n = ie(()=>t.when, void 0, void 0), r = e ? n : ie(n, void 0, {
            equals: (i, o)=>!i == !o
        });
        return ie(()=>{
            const i = r();
            if (i) {
                const o = t.children;
                return typeof o == "function" && o.length > 0 ? ye(()=>o(e ? i : ()=>{
                        if (!ye(r)) throw hi("Show");
                        return n();
                    })) : o;
            }
            return t.fallback;
        }, void 0, void 0);
    }
    function mi(t, e, n) {
        let r = n.length, i = e.length, o = r, s = 0, a = 0, l = e[i - 1].nextSibling, u = null;
        for(; s < i || a < o;){
            if (e[s] === n[a]) {
                s++, a++;
                continue;
            }
            for(; e[i - 1] === n[o - 1];)i--, o--;
            if (i === s) {
                const h = o < r ? a ? n[a - 1].nextSibling : n[o - a] : l;
                for(; a < o;)t.insertBefore(n[a++], h);
            } else if (o === a) for(; s < i;)(!u || !u.has(e[s])) && e[s].remove(), s++;
            else if (e[s] === n[o - 1] && n[a] === e[i - 1]) {
                const h = e[--i].nextSibling;
                t.insertBefore(n[a++], e[s++].nextSibling), t.insertBefore(n[--o], h), e[i] = n[o];
            } else {
                if (!u) {
                    u = new Map;
                    let d = a;
                    for(; d < o;)u.set(n[d], d++);
                }
                const h = u.get(e[s]);
                if (h != null) if (a < h && h < o) {
                    let d = s, _ = 1, f;
                    for(; ++d < i && d < o && !((f = u.get(e[d])) == null || f !== h + _);)_++;
                    if (_ > h - a) {
                        const m = e[s];
                        for(; a < h;)t.insertBefore(n[a++], m);
                    } else t.replaceChild(n[a++], e[s++]);
                } else s++;
                else e[s++].remove();
            }
        }
    }
    const vn = "_$DX_DELEGATE";
    function fi(t, e, n, r = {}) {
        let i;
        return rt((o)=>{
            i = o, e === document ? t() : R(e, t(), e.firstChild ? null : void 0, n);
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
    function ar(t, e = window.document) {
        const n = e[vn] || (e[vn] = new Set);
        for(let r = 0, i = t.length; r < i; r++){
            const o = t[r];
            n.has(o) || (n.add(o), e.addEventListener(o, gi));
        }
    }
    function Sn(t, e, n) {
        n == null ? t.removeAttribute(e) : t.setAttribute(e, n);
    }
    function Ie(t, e) {
        e == null ? t.removeAttribute("class") : t.className = e;
    }
    function pi(t, e, n, r) {
        if (Array.isArray(n)) {
            const i = n[0];
            t.addEventListener(e, n[0] = (o)=>i.call(t, n[1], o));
        } else t.addEventListener(e, n, typeof n != "function" && n);
    }
    function _i(t, e, n) {
        return ye(()=>t(e, n));
    }
    function R(t, e, n, r) {
        if (n !== void 0 && !r && (r = []), typeof e != "function") return ht(t, e, r, n);
        re((i)=>ht(t, e(), i, n), r);
    }
    function gi(t) {
        let e = t.target;
        const n = `$$${t.type}`, r = t.target, i = t.currentTarget, o = (l)=>Object.defineProperty(t, "target", {
                configurable: !0,
                value: l
            }), s = ()=>{
            const l = e[n];
            if (l && !e.disabled) {
                const u = e[`${n}Data`];
                if (u !== void 0 ? l.call(e, u, t) : l.call(e, t), t.cancelBubble) return;
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
            for(let u = 0; u < l.length - 2 && (e = l[u], !!s()); u++){
                if (e._$host) {
                    e = e._$host, a();
                    break;
                }
                if (e.parentNode === i) break;
            }
        } else a();
        o(r);
    }
    function ht(t, e, n, r, i) {
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
            if (o === "function") return re(()=>{
                let a = e();
                for(; typeof a == "function";)a = a();
                n = ht(t, a, n, r);
            }), ()=>n;
            if (Array.isArray(e)) {
                const a = [], l = n && Array.isArray(n);
                if (Ht(a, e, n, i)) return re(()=>n = ht(t, a, n, r, !0)), ()=>n;
                if (a.length === 0) {
                    if (n = ve(t, n, r), s) return n;
                } else l ? n.length === 0 ? An(t, a, r) : mi(t, n, a) : (n && ve(t), An(t, a));
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
    function Ht(t, e, n, r) {
        let i = !1;
        for(let o = 0, s = e.length; o < s; o++){
            let a = e[o], l = n && n[t.length], u;
            if (!(a == null || a === !0 || a === !1)) if ((u = typeof a) == "object" && a.nodeType) t.push(a);
            else if (Array.isArray(a)) i = Ht(t, a, l) || i;
            else if (u === "function") if (r) {
                for(; typeof a == "function";)a = a();
                i = Ht(t, Array.isArray(a) ? a : [
                    a
                ], Array.isArray(l) ? l : [
                    l
                ]) || i;
            } else t.push(a), i = !0;
            else {
                const h = String(a);
                l && l.nodeType === 3 && l.data === h ? t.push(l) : t.push(document.createTextNode(h));
            }
        }
        return i;
    }
    function An(t, e, n = null) {
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
    const yi = "/assets/markdown_binding_bg-BUSdgnPA.wasm", bi = async (t = {}, e)=>{
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
    let L;
    function Ei(t) {
        L = t;
    }
    const se = new Array(128).fill(void 0);
    se.push(void 0, null, !0, !1);
    function S(t) {
        return se[t];
    }
    let Te = 0, Je = null;
    function it() {
        return (Je === null || Je.byteLength === 0) && (Je = new Uint8Array(L.memory.buffer)), Je;
    }
    const wi = typeof TextEncoder > "u" ? (0, module.require)("util").TextEncoder : TextEncoder;
    let ot = new wi("utf-8");
    const vi = typeof ot.encodeInto == "function" ? function(t, e) {
        return ot.encodeInto(t, e);
    } : function(t, e) {
        const n = ot.encode(t);
        return e.set(n), {
            read: t.length,
            written: n.length
        };
    };
    function At(t, e, n) {
        if (n === void 0) {
            const a = ot.encode(t), l = e(a.length, 1) >>> 0;
            return it().subarray(l, l + a.length).set(a), Te = a.length, l;
        }
        let r = t.length, i = e(r, 1) >>> 0;
        const o = it();
        let s = 0;
        for(; s < r; s++){
            const a = t.charCodeAt(s);
            if (a > 127) break;
            o[i + s] = a;
        }
        if (s !== r) {
            s !== 0 && (t = t.slice(s)), i = n(i, r, r = s + t.length * 3, 1) >>> 0;
            const a = it().subarray(i + s, i + r), l = vi(t, a);
            s += l.written, i = n(i, r, s, 1) >>> 0;
        }
        return Te = s, i;
    }
    function Ve(t) {
        return t == null;
    }
    let Xe = null;
    function Q() {
        return (Xe === null || Xe.byteLength === 0) && (Xe = new Int32Array(L.memory.buffer)), Xe;
    }
    let xe = se.length;
    function Si(t) {
        t < 132 || (se[t] = xe, xe = t);
    }
    function me(t) {
        const e = S(t);
        return Si(t), e;
    }
    const Ai = typeof TextDecoder > "u" ? (0, module.require)("util").TextDecoder : TextDecoder;
    let lr = new Ai("utf-8", {
        ignoreBOM: !0,
        fatal: !0
    });
    lr.decode();
    function He(t, e) {
        return t = t >>> 0, lr.decode(it().subarray(t, t + e));
    }
    function C(t) {
        xe === se.length && se.push(se.length + 1);
        const e = xe;
        return xe = se[e], se[e] = t, e;
    }
    let Ye = null;
    function Ti() {
        return (Ye === null || Ye.byteLength === 0) && (Ye = new BigInt64Array(L.memory.buffer)), Ye;
    }
    let Qe = null;
    function Ri() {
        return (Qe === null || Qe.byteLength === 0) && (Qe = new Float64Array(L.memory.buffer)), Qe;
    }
    function Ft(t) {
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
            i > 0 && (o += Ft(t[0]));
            for(let s = 1; s < i; s++)o += ", " + Ft(t[s]);
            return o += "]", o;
        }
        const n = /\[object ([^\]]+)\]/.exec(toString.call(t));
        let r;
        if (n.length > 1) r = n[1];
        else return toString.call(t);
        if (r == "Object") try {
            return "Object(" + JSON.stringify(t) + ")";
        } catch  {
            return "Object";
        }
        return t instanceof Error ? `${t.name}: ${t.message}
${t.stack}` : r;
    }
    function Li(t, e) {
        const n = At(t, L.__wbindgen_export_0, L.__wbindgen_export_1), r = Te, i = L.parse_with_options(n, r, C(e));
        return on.__wrap(i);
    }
    function Tt(t, e) {
        try {
            return t.apply(this, e);
        } catch (n) {
            L.__wbindgen_export_3(C(n));
        }
    }
    const Tn = typeof FinalizationRegistry > "u" ? {
        register: ()=>{},
        unregister: ()=>{}
    } : new FinalizationRegistry((t)=>L.__wbg_document_free(t >>> 0));
    class on {
        static __wrap(e) {
            e = e >>> 0;
            const n = Object.create(on.prototype);
            return n.__wbg_ptr = e, Tn.register(n, n.__wbg_ptr, n), n;
        }
        __destroy_into_raw() {
            const e = this.__wbg_ptr;
            return this.__wbg_ptr = 0, Tn.unregister(this), e;
        }
        free() {
            const e = this.__destroy_into_raw();
            L.__wbg_document_free(e);
        }
        get frontmatter() {
            const e = L.document_frontmatter(this.__wbg_ptr);
            return me(e);
        }
        get total_nodes() {
            return L.document_total_nodes(this.__wbg_ptr) >>> 0;
        }
        continue_parse() {
            try {
                const r = L.__wbindgen_add_to_stack_pointer(-16);
                L.document_continue_parse(r, this.__wbg_ptr);
                var e = Q()[r / 4 + 0], n = Q()[r / 4 + 1];
                if (n) throw me(e);
            } finally{
                L.__wbindgen_add_to_stack_pointer(16);
            }
        }
        get tags() {
            const e = L.document_tags(this.__wbg_ptr);
            return me(e);
        }
        get tree() {
            const e = L.document_tree(this.__wbg_ptr);
            return me(e);
        }
        to_html() {
            let e, n;
            try {
                const o = L.__wbindgen_add_to_stack_pointer(-16);
                L.document_to_html(o, this.__wbg_ptr);
                var r = Q()[o / 4 + 0], i = Q()[o / 4 + 1];
                return e = r, n = i, He(r, i);
            } finally{
                L.__wbindgen_add_to_stack_pointer(16), L.__wbindgen_export_2(e, n, 1);
            }
        }
    }
    function Pi(t, e) {
        const n = S(e), r = typeof n == "string" ? n : void 0;
        var i = Ve(r) ? 0 : At(r, L.__wbindgen_export_0, L.__wbindgen_export_1), o = Te;
        Q()[t / 4 + 1] = o, Q()[t / 4 + 0] = i;
    }
    function ki(t) {
        me(t);
    }
    function Ci(t, e) {
        return S(t) == S(e);
    }
    function Ii(t, e) {
        const n = He(t, e);
        return C(n);
    }
    function Oi() {
        const t = new Object;
        return C(t);
    }
    function xi(t, e, n) {
        S(t)[me(e)] = me(n);
    }
    function Di() {
        const t = new Array;
        return C(t);
    }
    function Ni(t) {
        return C(t);
    }
    function Vi(t, e, n) {
        S(t)[e >>> 0] = me(n);
    }
    function Mi(t) {
        const e = BigInt.asUintN(64, t);
        return C(e);
    }
    function $i() {
        return Tt(function(t) {
            const e = String.fromCodePoint(t >>> 0);
            return C(e);
        }, arguments);
    }
    function ji(t) {
        const e = S(t);
        return typeof e == "object" && e !== null;
    }
    function Bi(t, e) {
        const n = S(t)[S(e)];
        return C(n);
    }
    function Gi(t) {
        return S(t) === void 0;
    }
    function Ui(t, e) {
        return S(t) in S(e);
    }
    function Hi(t) {
        return Array.isArray(S(t));
    }
    function Fi(t) {
        return S(t).length;
    }
    function Wi(t, e) {
        const n = S(t)[e >>> 0];
        return C(n);
    }
    function zi() {
        return C(Symbol.iterator);
    }
    function qi() {
        return Tt(function(t, e) {
            const n = Reflect.get(S(t), S(e));
            return C(n);
        }, arguments);
    }
    function Ki(t) {
        return typeof S(t) == "function";
    }
    function Ji() {
        return Tt(function(t, e) {
            const n = S(t).call(S(e));
            return C(n);
        }, arguments);
    }
    function Xi(t) {
        const e = S(t).next;
        return C(e);
    }
    function Yi() {
        return Tt(function(t) {
            const e = S(t).next();
            return C(e);
        }, arguments);
    }
    function Qi(t) {
        return S(t).done;
    }
    function Zi(t) {
        const e = S(t).value;
        return C(e);
    }
    function eo(t) {
        const e = S(t);
        return typeof e == "boolean" ? e ? 1 : 0 : 2;
    }
    function to(t) {
        return typeof S(t) == "string";
    }
    function no(t) {
        const e = Object.entries(S(t));
        return C(e);
    }
    function ro() {
        const t = new Error;
        return C(t);
    }
    function io(t, e) {
        const n = S(e).stack, r = At(n, L.__wbindgen_export_0, L.__wbindgen_export_1), i = Te;
        Q()[t / 4 + 1] = i, Q()[t / 4 + 0] = r;
    }
    function oo(t, e) {
        let n, r;
        try {
            n = t, r = e, console.error(He(t, e));
        } finally{
            L.__wbindgen_export_2(n, r, 1);
        }
    }
    function so(t) {
        return S(t).length;
    }
    function ao() {
        const t = L.memory;
        return C(t);
    }
    function lo(t) {
        const e = S(t).buffer;
        return C(e);
    }
    function co(t) {
        const e = new Uint8Array(S(t));
        return C(e);
    }
    function uo(t, e, n) {
        S(t).set(S(e), n >>> 0);
    }
    function ho() {
        return C(new Map);
    }
    function mo(t, e, n) {
        const r = S(t).set(S(e), S(n));
        return C(r);
    }
    function fo(t, e) {
        const n = new Error(He(t, e));
        return C(n);
    }
    function po(t) {
        return typeof S(t) == "bigint";
    }
    function _o(t) {
        return Number.isSafeInteger(S(t));
    }
    function go(t) {
        return +S(t);
    }
    function yo(t, e) {
        const n = S(e), r = typeof n == "bigint" ? n : void 0;
        Ti()[t / 8 + 1] = Ve(r) ? BigInt(0) : r, Q()[t / 4 + 0] = !Ve(r);
    }
    function bo(t, e) {
        return S(t) === S(e);
    }
    function Eo(t) {
        return C(t);
    }
    function wo(t) {
        const e = S(t);
        return C(e);
    }
    function vo(t, e) {
        const n = S(e), r = typeof n == "number" ? n : void 0;
        Ri()[t / 8 + 1] = Ve(r) ? 0 : r, Q()[t / 4 + 0] = !Ve(r);
    }
    function So(t) {
        let e;
        try {
            e = S(t) instanceof Uint8Array;
        } catch  {
            e = !1;
        }
        return e;
    }
    function Ao(t) {
        let e;
        try {
            e = S(t) instanceof ArrayBuffer;
        } catch  {
            e = !1;
        }
        return e;
    }
    function To(t, e) {
        throw new Error(He(t, e));
    }
    function Ro(t, e) {
        const n = Ft(S(e)), r = At(n, L.__wbindgen_export_0, L.__wbindgen_export_1), i = Te;
        Q()[t / 4 + 1] = i, Q()[t / 4 + 0] = r;
    }
    URL = globalThis.URL;
    const F = await bi({
        "./markdown_binding_bg.js": {
            __wbindgen_string_get: Pi,
            __wbindgen_object_drop_ref: ki,
            __wbindgen_jsval_loose_eq: Ci,
            __wbindgen_string_new: Ii,
            __wbg_new_72fb9a18b5ae2624: Oi,
            __wbg_set_f975102236d3c502: xi,
            __wbg_new_16b304a2cfa7ff4a: Di,
            __wbindgen_number_new: Ni,
            __wbg_set_d4638f722068f043: Vi,
            __wbindgen_bigint_from_u64: Mi,
            __wbg_fromCodePoint_cedd7612a2ff688f: $i,
            __wbindgen_is_object: ji,
            __wbg_getwithrefkey_edc2c8960f0f1191: Bi,
            __wbindgen_is_undefined: Gi,
            __wbindgen_in: Ui,
            __wbg_isArray_2ab64d95e09ea0ae: Hi,
            __wbg_length_cd7af8117672b8b8: Fi,
            __wbg_get_bd8e338fbd5f5cc8: Wi,
            __wbg_iterator_2cee6dadfd956dfa: zi,
            __wbg_get_e3c254076557e348: qi,
            __wbindgen_is_function: Ki,
            __wbg_call_27c0f87801dedf93: Ji,
            __wbg_next_40fc327bfc8770e6: Xi,
            __wbg_next_196c84450b364254: Yi,
            __wbg_done_298b57d23c0fc80c: Qi,
            __wbg_value_d93c65011f51a456: Zi,
            __wbindgen_boolean_get: eo,
            __wbindgen_is_string: to,
            __wbg_entries_95cc2c823b285a09: no,
            __wbg_new_abda76e883ba8a5f: ro,
            __wbg_stack_658279fe44541cf6: io,
            __wbg_error_f851667af71bcfc6: oo,
            __wbg_length_c20a40f15020d68a: so,
            __wbindgen_memory: ao,
            __wbg_buffer_12d079cc21e14bdb: lo,
            __wbg_new_63b92bc8671ed464: co,
            __wbg_set_a47bac70306a19a7: uo,
            __wbg_new_d9bc3a0147634640: ho,
            __wbg_set_8417257aaedc936b: mo,
            __wbindgen_error_new: fo,
            __wbindgen_is_bigint: po,
            __wbg_isSafeInteger_f7b04ef02296c4d2: _o,
            __wbindgen_as_number: go,
            __wbindgen_bigint_get_as_i64: yo,
            __wbindgen_jsval_eq: bo,
            __wbindgen_bigint_from_i64: Eo,
            __wbindgen_object_clone_ref: wo,
            __wbindgen_number_get: vo,
            __wbg_instanceof_Uint8Array_2b3bbecd033d19f6: So,
            __wbg_instanceof_ArrayBuffer_836825be07d4c9d2: Ao,
            __wbindgen_throw: To,
            __wbindgen_debug_string: Ro
        }
    }, yi), Lo = F.memory, Po = F.__wbg_document_free, ko = F.document_continue_parse, Co = F.document_frontmatter, Io = F.document_tags, Oo = F.document_to_html, xo = F.document_total_nodes, Do = F.document_tree, No = F.parse, Vo = F.parse_with_options, Mo = F.version, $o = F.__wbindgen_export_0, jo = F.__wbindgen_export_1, Bo = F.__wbindgen_add_to_stack_pointer, Go = F.__wbindgen_export_2, Uo = F.__wbindgen_export_3, Ho = Object.freeze(Object.defineProperty({
        __proto__: null,
        __wbg_document_free: Po,
        __wbindgen_add_to_stack_pointer: Bo,
        __wbindgen_export_0: $o,
        __wbindgen_export_1: jo,
        __wbindgen_export_2: Go,
        __wbindgen_export_3: Uo,
        document_continue_parse: ko,
        document_frontmatter: Co,
        document_tags: Io,
        document_to_html: Oo,
        document_total_nodes: xo,
        document_tree: Do,
        memory: Lo,
        parse: No,
        parse_with_options: Vo,
        version: Mo
    }, Symbol.toStringTag, {
        value: "Module"
    }));
    Ei(Ho);
    var Fo = N("<span class=json-array-label>"), Wo = N("<span class=json-colon> "), zo = N("<span class=json-preview> <!> "), qo = N("<span class=json-bracket>"), Dt = N("<span class=json-comma>,"), Ko = N('<div class=json-line><button class=json-toggle><svg width=12 height=12 viewBox="0 0 12 12"><path d="M4 2 L8 6 L4 10"fill=none stroke=currentColor stroke-width=1.5></path></svg></button><span class=json-bracket>'), Jo = N("<div class=json-children>"), Xo = N('<div class="json-line json-closing-bracket"><span class=json-spacer></span><span class=json-bracket>'), Rn = N('<span class=json-key>"<!>"'), Ln = N("<span class=json-colon>: "), Yo = N("<div class=json-line><span class=json-spacer></span><span>");
    function Wt(t) {
        const [e, n] = ee(t.depth ? t.depth > 1 : !1), r = t.depth || 0, i = (m)=>m !== null && typeof m == "object" && !Array.isArray(m), o = (m)=>Array.isArray(m), s = (m)=>!i(m) && !o(m), a = (m)=>m === null ? "json-null" : m === void 0 ? "json-undefined" : typeof m == "string" ? "json-string" : typeof m == "number" ? "json-number" : typeof m == "boolean" ? "json-boolean" : "", l = (m)=>m === null ? "null" : m === void 0 ? "undefined" : typeof m == "string" ? `"${m}"` : String(m), u = (m)=>{
            if (o(m)) return m.length === 0 ? "" : `${m.length} items`;
            if (i(m)) {
                const w = Object.keys(m);
                return w.length === 0 ? "" : `${w.length} keys`;
            }
            return "";
        }, h = ()=>{
            if (t.arrayIndex !== void 0 && i(t.data) && t.data.kind) return t.data.kind;
        }, d = ie(()=>i(t.data) ? Object.entries(t.data) : o(t.data) ? t.data.map((m, w)=>[
                    w,
                    m
                ]) : []), _ = (m)=>{
            m.stopPropagation(), t.onNodeClick && i(t.data) && t.onNodeClick(t.data.start, t.data.end);
        }, f = t.name || h();
        return [
            I(G, {
                get when () {
                    return i(t.data) || o(t.data);
                },
                get children () {
                    return [
                        (()=>{
                            var m = Ko(), w = m.firstChild, g = w.firstChild, E = w.nextSibling;
                            return m.$$click = _, w.$$click = (p)=>{
                                p.stopPropagation(), n(!e());
                            }, R(m, I(G, {
                                when: f !== void 0,
                                get children () {
                                    return I(G, {
                                        get when () {
                                            return t.arrayIndex !== void 0;
                                        },
                                        get fallback () {
                                            return [
                                                (()=>{
                                                    var p = Rn(), b = p.firstChild, v = b.nextSibling;
                                                    return v.nextSibling, R(p, f, v), p;
                                                })(),
                                                Ln()
                                            ];
                                        },
                                        get children () {
                                            return [
                                                (()=>{
                                                    var p = Fo();
                                                    return R(p, f), p;
                                                })(),
                                                Wo()
                                            ];
                                        }
                                    });
                                }
                            }), E), R(E, ()=>o(t.data) ? "[" : "{"), R(m, I(G, {
                                get when () {
                                    return e();
                                },
                                get children () {
                                    return [
                                        (()=>{
                                            var p = zo(), b = p.firstChild, v = b.nextSibling;
                                            return v.nextSibling, R(p, ()=>u(t.data), v), p;
                                        })(),
                                        (()=>{
                                            var p = qo();
                                            return R(p, ()=>o(t.data) ? "]" : "}"), p;
                                        })()
                                    ];
                                }
                            }), null), R(m, I(G, {
                                get when () {
                                    return ie(()=>!t.isLast)() && e();
                                },
                                get children () {
                                    return Dt();
                                }
                            }), null), re((p)=>{
                                var b = e() ? "Expand" : "Collapse", v = `json-arrow ${e() ? "json-arrow-collapsed" : "json-arrow-expanded"}`;
                                return b !== p.e && Sn(w, "title", p.e = b), v !== p.t && Sn(g, "class", p.t = v), p;
                            }, {
                                e: void 0,
                                t: void 0
                            }), m;
                        })(),
                        I(G, {
                            get when () {
                                return !e();
                            },
                            get children () {
                                return [
                                    (()=>{
                                        var m = Jo();
                                        return R(m, I(sr, {
                                            get each () {
                                                return d();
                                            },
                                            children: ([w, g], E)=>I(Wt, {
                                                    get name () {
                                                        return ie(()=>!!o(t.data))() ? void 0 : String(w);
                                                    },
                                                    data: g,
                                                    depth: r + 1,
                                                    get isLast () {
                                                        return E() === d().length - 1;
                                                    },
                                                    get onNodeClick () {
                                                        return t.onNodeClick;
                                                    },
                                                    get arrayIndex () {
                                                        return ie(()=>!!o(t.data))() ? Number(w) : void 0;
                                                    }
                                                })
                                        })), m;
                                    })(),
                                    (()=>{
                                        var m = Xo(), w = m.firstChild, g = w.nextSibling;
                                        return R(g, ()=>o(t.data) ? "]" : "}"), R(m, I(G, {
                                            get when () {
                                                return !t.isLast;
                                            },
                                            get children () {
                                                return Dt();
                                            }
                                        }), null), m;
                                    })()
                                ];
                            }
                        })
                    ];
                }
            }),
            I(G, {
                get when () {
                    return s(t.data);
                },
                get children () {
                    var m = Yo(), w = m.firstChild, g = w.nextSibling;
                    return R(m, I(G, {
                        get when () {
                            return t.name !== void 0;
                        },
                        get children () {
                            return [
                                (()=>{
                                    var E = Rn(), p = E.firstChild, b = p.nextSibling;
                                    return b.nextSibling, R(E, ()=>t.name, b), E;
                                })(),
                                Ln()
                            ];
                        }
                    }), g), R(g, ()=>l(t.data)), R(m, I(G, {
                        get when () {
                            return !t.isLast;
                        },
                        get children () {
                            return Dt();
                        }
                    }), null), re(()=>Ie(g, a(t.data))), m;
                }
            })
        ];
    }
    ar([
        "click"
    ]);
    const Qo = "modulepreload", Zo = function(t) {
        return "/" + t;
    }, Pn = {}, c = function(e, n, r) {
        let i = Promise.resolve();
        if (n && n.length > 0) {
            document.getElementsByTagName("link");
            const s = document.querySelector("meta[property=csp-nonce]"), a = s?.nonce || s?.getAttribute("nonce");
            i = Promise.allSettled(n.map((l)=>{
                if (l = Zo(l), l in Pn) return;
                Pn[l] = !0;
                const u = l.endsWith(".css"), h = u ? '[rel="stylesheet"]' : "";
                if (document.querySelector(`link[href="${l}"]${h}`)) return;
                const d = document.createElement("link");
                if (d.rel = u ? "stylesheet" : Qo, u || (d.as = "script"), d.crossOrigin = "", d.href = l, a && d.setAttribute("nonce", a), document.head.appendChild(d), u) return new Promise((_, f)=>{
                    d.addEventListener("load", _), d.addEventListener("error", ()=>f(new Error(`Unable to preload CSS for ${l}`)));
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
    }, cr = [
        {
            id: "abap",
            name: "ABAP",
            import: ()=>c(()=>import("./abap-DsBKuouk.js"), [])
        },
        {
            id: "actionscript-3",
            name: "ActionScript",
            import: ()=>c(()=>import("./actionscript-3-D_z4Izcz.js"), [])
        },
        {
            id: "ada",
            name: "Ada",
            import: ()=>c(()=>import("./ada-727ZlQH0.js"), [])
        },
        {
            id: "angular-html",
            name: "Angular HTML",
            import: ()=>c(()=>import("./angular-html-LfdN0zeE.js").then((t)=>t.f), __vite__mapDeps([0,1,2,3]))
        },
        {
            id: "angular-ts",
            name: "Angular TypeScript",
            import: ()=>c(()=>import("./angular-ts-CKsD7JZE.js"), __vite__mapDeps([4,0,1,2,3,5]))
        },
        {
            id: "apache",
            name: "Apache Conf",
            import: ()=>c(()=>import("./apache-Dn00JSTd.js"), [])
        },
        {
            id: "apex",
            name: "Apex",
            import: ()=>c(()=>import("./apex-COJ4H7py.js"), [])
        },
        {
            id: "apl",
            name: "APL",
            import: ()=>c(()=>import("./apl-BBq3IX1j.js"), __vite__mapDeps([6,1,2,3,7,8,9]))
        },
        {
            id: "applescript",
            name: "AppleScript",
            import: ()=>c(()=>import("./applescript-Bu5BbsvL.js"), [])
        },
        {
            id: "ara",
            name: "Ara",
            import: ()=>c(()=>import("./ara-7O62HKoU.js"), [])
        },
        {
            id: "asciidoc",
            name: "AsciiDoc",
            aliases: [
                "adoc"
            ],
            import: ()=>c(()=>import("./asciidoc-BPT9niGB.js"), [])
        },
        {
            id: "asm",
            name: "Assembly",
            import: ()=>c(()=>import("./asm-Dhn9LcZ4.js"), [])
        },
        {
            id: "astro",
            name: "Astro",
            import: ()=>c(()=>import("./astro-CqkE3fuf.js"), __vite__mapDeps([10,9,2,11,3,12]))
        },
        {
            id: "awk",
            name: "AWK",
            import: ()=>c(()=>import("./awk-eg146-Ew.js"), [])
        },
        {
            id: "ballerina",
            name: "Ballerina",
            import: ()=>c(()=>import("./ballerina-Du268qiB.js"), [])
        },
        {
            id: "bat",
            name: "Batch File",
            aliases: [
                "batch"
            ],
            import: ()=>c(()=>import("./bat-fje9CFhw.js"), [])
        },
        {
            id: "beancount",
            name: "Beancount",
            import: ()=>c(()=>import("./beancount-BwXTMy5W.js"), [])
        },
        {
            id: "berry",
            name: "Berry",
            aliases: [
                "be"
            ],
            import: ()=>c(()=>import("./berry-3xVqZejG.js"), [])
        },
        {
            id: "bibtex",
            name: "BibTeX",
            import: ()=>c(()=>import("./bibtex-xW4inM5L.js"), [])
        },
        {
            id: "bicep",
            name: "Bicep",
            import: ()=>c(()=>import("./bicep-DHo0CJ0O.js"), [])
        },
        {
            id: "blade",
            name: "Blade",
            import: ()=>c(()=>import("./blade-a8OxSdnT.js"), __vite__mapDeps([13,1,2,3,7,8,14,9]))
        },
        {
            id: "bsl",
            name: "1C (Enterprise)",
            aliases: [
                "1c"
            ],
            import: ()=>c(()=>import("./bsl-Dgyn0ogV.js"), __vite__mapDeps([15,16]))
        },
        {
            id: "c",
            name: "C",
            import: ()=>c(()=>import("./c-C3t2pwGQ.js"), [])
        },
        {
            id: "cadence",
            name: "Cadence",
            aliases: [
                "cdc"
            ],
            import: ()=>c(()=>import("./cadence-DNquZEk8.js"), [])
        },
        {
            id: "cairo",
            name: "Cairo",
            import: ()=>c(()=>import("./cairo--RitsXJZ.js"), __vite__mapDeps([17,18]))
        },
        {
            id: "clarity",
            name: "Clarity",
            import: ()=>c(()=>import("./clarity-BHOwM8T6.js"), [])
        },
        {
            id: "clojure",
            name: "Clojure",
            aliases: [
                "clj"
            ],
            import: ()=>c(()=>import("./clojure-DxSadP1t.js"), [])
        },
        {
            id: "cmake",
            name: "CMake",
            import: ()=>c(()=>import("./cmake-DbXoA79R.js"), [])
        },
        {
            id: "cobol",
            name: "COBOL",
            import: ()=>c(()=>import("./cobol-PTqiYgYu.js"), __vite__mapDeps([19,1,2,3,8]))
        },
        {
            id: "codeowners",
            name: "CODEOWNERS",
            import: ()=>c(()=>import("./codeowners-Bp6g37R7.js"), [])
        },
        {
            id: "codeql",
            name: "CodeQL",
            aliases: [
                "ql"
            ],
            import: ()=>c(()=>import("./codeql-sacFqUAJ.js"), [])
        },
        {
            id: "coffee",
            name: "CoffeeScript",
            aliases: [
                "coffeescript"
            ],
            import: ()=>c(()=>import("./coffee-dyiR41kL.js"), __vite__mapDeps([20,2]))
        },
        {
            id: "common-lisp",
            name: "Common Lisp",
            aliases: [
                "lisp"
            ],
            import: ()=>c(()=>import("./common-lisp-C7gG9l05.js"), [])
        },
        {
            id: "coq",
            name: "Coq",
            import: ()=>c(()=>import("./coq-Dsg_Bt_b.js"), [])
        },
        {
            id: "cpp",
            name: "C++",
            aliases: [
                "c++"
            ],
            import: ()=>c(()=>import("./cpp-BksuvNSY.js"), __vite__mapDeps([21,22,23,24,14]))
        },
        {
            id: "crystal",
            name: "Crystal",
            import: ()=>c(()=>import("./crystal-DtDmRg-F.js"), __vite__mapDeps([25,1,2,3,14,24,26]))
        },
        {
            id: "csharp",
            name: "C#",
            aliases: [
                "c#",
                "cs"
            ],
            import: ()=>c(()=>import("./csharp-D9R-vmeu.js"), [])
        },
        {
            id: "css",
            name: "CSS",
            import: ()=>c(()=>import("./css-BPhBrDlE.js"), [])
        },
        {
            id: "csv",
            name: "CSV",
            import: ()=>c(()=>import("./csv-B0qRVHPH.js"), [])
        },
        {
            id: "cue",
            name: "CUE",
            import: ()=>c(()=>import("./cue-DtFQj3wx.js"), [])
        },
        {
            id: "cypher",
            name: "Cypher",
            aliases: [
                "cql"
            ],
            import: ()=>c(()=>import("./cypher-m2LEI-9-.js"), [])
        },
        {
            id: "d",
            name: "D",
            import: ()=>c(()=>import("./d-BoXegm-a.js"), [])
        },
        {
            id: "dart",
            name: "Dart",
            import: ()=>c(()=>import("./dart-B9wLZaAG.js"), [])
        },
        {
            id: "dax",
            name: "DAX",
            import: ()=>c(()=>import("./dax-ClGRhx96.js"), [])
        },
        {
            id: "desktop",
            name: "Desktop",
            import: ()=>c(()=>import("./desktop-DEIpsLCJ.js"), [])
        },
        {
            id: "diff",
            name: "Diff",
            import: ()=>c(()=>import("./diff-BgYniUM_.js"), [])
        },
        {
            id: "docker",
            name: "Dockerfile",
            aliases: [
                "dockerfile"
            ],
            import: ()=>c(()=>import("./docker-COcR7UxN.js"), [])
        },
        {
            id: "dotenv",
            name: "dotEnv",
            import: ()=>c(()=>import("./dotenv-BjQB5zDj.js"), [])
        },
        {
            id: "dream-maker",
            name: "Dream Maker",
            import: ()=>c(()=>import("./dream-maker-C-nORZOA.js"), [])
        },
        {
            id: "edge",
            name: "Edge",
            import: ()=>c(()=>import("./edge-D5gP-w-T.js"), __vite__mapDeps([27,11,1,2,3,28]))
        },
        {
            id: "elixir",
            name: "Elixir",
            import: ()=>c(()=>import("./elixir-CLiX3zqd.js"), __vite__mapDeps([29,1,2,3]))
        },
        {
            id: "elm",
            name: "Elm",
            import: ()=>c(()=>import("./elm-CmHSxxaM.js"), __vite__mapDeps([30,23,24]))
        },
        {
            id: "emacs-lisp",
            name: "Emacs Lisp",
            aliases: [
                "elisp"
            ],
            import: ()=>c(()=>import("./emacs-lisp-BX77sIaO.js"), [])
        },
        {
            id: "erb",
            name: "ERB",
            import: ()=>c(()=>import("./erb-BYTLMnw6.js"), __vite__mapDeps([31,1,2,3,32,33,7,8,14,34,11,35,36,21,22,23,24,26,37,38]))
        },
        {
            id: "erlang",
            name: "Erlang",
            aliases: [
                "erl"
            ],
            import: ()=>c(()=>import("./erlang-B-DoSBHF.js"), [])
        },
        {
            id: "fennel",
            name: "Fennel",
            import: ()=>c(()=>import("./fennel-bCA53EVm.js"), [])
        },
        {
            id: "fish",
            name: "Fish",
            import: ()=>c(()=>import("./fish-w-ucz2PV.js"), [])
        },
        {
            id: "fluent",
            name: "Fluent",
            aliases: [
                "ftl"
            ],
            import: ()=>c(()=>import("./fluent-Dayu4EKP.js"), [])
        },
        {
            id: "fortran-fixed-form",
            name: "Fortran (Fixed Form)",
            aliases: [
                "f",
                "for",
                "f77"
            ],
            import: ()=>c(()=>import("./fortran-fixed-form-TqA4NnZg.js"), __vite__mapDeps([39,40]))
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
            import: ()=>c(()=>import("./fortran-free-form-DKXYxT9g.js"), [])
        },
        {
            id: "fsharp",
            name: "F#",
            aliases: [
                "f#",
                "fs"
            ],
            import: ()=>c(()=>import("./fsharp-XplgxFYe.js"), __vite__mapDeps([41,42]))
        },
        {
            id: "gdresource",
            name: "GDResource",
            import: ()=>c(()=>import("./gdresource-BHYsBjWJ.js"), __vite__mapDeps([43,44,45]))
        },
        {
            id: "gdscript",
            name: "GDScript",
            import: ()=>c(()=>import("./gdscript-DfxzS6Rs.js"), [])
        },
        {
            id: "gdshader",
            name: "GDShader",
            import: ()=>c(()=>import("./gdshader-SKMF96pI.js"), [])
        },
        {
            id: "genie",
            name: "Genie",
            import: ()=>c(()=>import("./genie-ajMbGru0.js"), [])
        },
        {
            id: "gherkin",
            name: "Gherkin",
            import: ()=>c(()=>import("./gherkin--30QC5Em.js"), [])
        },
        {
            id: "git-commit",
            name: "Git Commit Message",
            import: ()=>c(()=>import("./git-commit-i4q6IMui.js"), __vite__mapDeps([46,47]))
        },
        {
            id: "git-rebase",
            name: "Git Rebase Message",
            import: ()=>c(()=>import("./git-rebase-B-v9cOL2.js"), __vite__mapDeps([48,26]))
        },
        {
            id: "gleam",
            name: "Gleam",
            import: ()=>c(()=>import("./gleam-B430Bg39.js"), [])
        },
        {
            id: "glimmer-js",
            name: "Glimmer JS",
            aliases: [
                "gjs"
            ],
            import: ()=>c(()=>import("./glimmer-js-D-cwc0-E.js"), __vite__mapDeps([49,2,11,3,1]))
        },
        {
            id: "glimmer-ts",
            name: "Glimmer TS",
            aliases: [
                "gts"
            ],
            import: ()=>c(()=>import("./glimmer-ts-pgjy16dm.js"), __vite__mapDeps([50,11,3,2,1]))
        },
        {
            id: "glsl",
            name: "GLSL",
            import: ()=>c(()=>import("./glsl-DBO2IWDn.js"), __vite__mapDeps([23,24]))
        },
        {
            id: "gnuplot",
            name: "Gnuplot",
            import: ()=>c(()=>import("./gnuplot-CM8KxXT1.js"), [])
        },
        {
            id: "go",
            name: "Go",
            import: ()=>c(()=>import("./go-B1SYOhNW.js"), [])
        },
        {
            id: "graphql",
            name: "GraphQL",
            aliases: [
                "gql"
            ],
            import: ()=>c(()=>import("./graphql-cDcHW_If.js"), __vite__mapDeps([34,2,11,35,36]))
        },
        {
            id: "groovy",
            name: "Groovy",
            import: ()=>c(()=>import("./groovy-DkBy-JyN.js"), [])
        },
        {
            id: "hack",
            name: "Hack",
            import: ()=>c(()=>import("./hack-D1yCygmZ.js"), __vite__mapDeps([51,1,2,3,14]))
        },
        {
            id: "haml",
            name: "Ruby Haml",
            import: ()=>c(()=>import("./haml-B2EZWmdv.js"), __vite__mapDeps([33,2,3]))
        },
        {
            id: "handlebars",
            name: "Handlebars",
            aliases: [
                "hbs"
            ],
            import: ()=>c(()=>import("./handlebars-BQGss363.js"), __vite__mapDeps([52,1,2,3,38]))
        },
        {
            id: "haskell",
            name: "Haskell",
            aliases: [
                "hs"
            ],
            import: ()=>c(()=>import("./haskell-BILxekzW.js"), [])
        },
        {
            id: "haxe",
            name: "Haxe",
            import: ()=>c(()=>import("./haxe-C5wWYbrZ.js"), [])
        },
        {
            id: "hcl",
            name: "HashiCorp HCL",
            import: ()=>c(()=>import("./hcl-HzYwdGDm.js"), [])
        },
        {
            id: "hjson",
            name: "Hjson",
            import: ()=>c(()=>import("./hjson-T-Tgc4AT.js"), [])
        },
        {
            id: "hlsl",
            name: "HLSL",
            import: ()=>c(()=>import("./hlsl-ifBTmRxC.js"), [])
        },
        {
            id: "html",
            name: "HTML",
            import: ()=>c(()=>import("./html-C2L_23MC.js"), __vite__mapDeps([1,2,3]))
        },
        {
            id: "html-derivative",
            name: "HTML (Derivative)",
            import: ()=>c(()=>import("./html-derivative-CSfWNPLT.js"), __vite__mapDeps([28,1,2,3]))
        },
        {
            id: "http",
            name: "HTTP",
            import: ()=>c(()=>import("./http-FRrOvY1W.js"), __vite__mapDeps([53,26,9,7,8,34,2,11,35,36]))
        },
        {
            id: "hxml",
            name: "HXML",
            import: ()=>c(()=>import("./hxml-TIA70rKU.js"), __vite__mapDeps([54,55]))
        },
        {
            id: "hy",
            name: "Hy",
            import: ()=>c(()=>import("./hy-BMj5Y0dO.js"), [])
        },
        {
            id: "imba",
            name: "Imba",
            import: ()=>c(()=>import("./imba-bv_oIlVt.js"), __vite__mapDeps([56,11]))
        },
        {
            id: "ini",
            name: "INI",
            aliases: [
                "properties"
            ],
            import: ()=>c(()=>import("./ini-BjABl1g7.js"), [])
        },
        {
            id: "java",
            name: "Java",
            import: ()=>c(()=>import("./java-xI-RfyKK.js"), [])
        },
        {
            id: "javascript",
            name: "JavaScript",
            aliases: [
                "js"
            ],
            import: ()=>c(()=>import("./javascript-ySlJ1b_l.js"), [])
        },
        {
            id: "jinja",
            name: "Jinja",
            import: ()=>c(()=>import("./jinja-DGy0s7-h.js"), __vite__mapDeps([57,1,2,3]))
        },
        {
            id: "jison",
            name: "Jison",
            import: ()=>c(()=>import("./jison-BqZprYcd.js"), __vite__mapDeps([58,2]))
        },
        {
            id: "json",
            name: "JSON",
            import: ()=>c(()=>import("./json-BQoSv7ci.js"), [])
        },
        {
            id: "json5",
            name: "JSON5",
            import: ()=>c(()=>import("./json5-w8dY5SsB.js"), [])
        },
        {
            id: "jsonc",
            name: "JSON with Comments",
            import: ()=>c(()=>import("./jsonc-TU54ms6u.js"), [])
        },
        {
            id: "jsonl",
            name: "JSON Lines",
            import: ()=>c(()=>import("./jsonl-DREVFZK8.js"), [])
        },
        {
            id: "jsonnet",
            name: "Jsonnet",
            import: ()=>c(()=>import("./jsonnet-BfivnA6A.js"), [])
        },
        {
            id: "jssm",
            name: "JSSM",
            aliases: [
                "fsl"
            ],
            import: ()=>c(()=>import("./jssm-P4WzXJd0.js"), [])
        },
        {
            id: "jsx",
            name: "JSX",
            import: ()=>c(()=>import("./jsx-BAng5TT0.js"), [])
        },
        {
            id: "julia",
            name: "Julia",
            aliases: [
                "jl"
            ],
            import: ()=>c(()=>import("./julia-BBuGR-5E.js"), __vite__mapDeps([59,21,22,23,24,14,18,2,60]))
        },
        {
            id: "kotlin",
            name: "Kotlin",
            aliases: [
                "kt",
                "kts"
            ],
            import: ()=>c(()=>import("./kotlin-B5lbUyaz.js"), [])
        },
        {
            id: "kusto",
            name: "Kusto",
            aliases: [
                "kql"
            ],
            import: ()=>c(()=>import("./kusto-mebxcVVE.js"), [])
        },
        {
            id: "latex",
            name: "LaTeX",
            import: ()=>c(()=>import("./latex-C-cWTeAZ.js"), __vite__mapDeps([61,62,60]))
        },
        {
            id: "lean",
            name: "Lean 4",
            aliases: [
                "lean4"
            ],
            import: ()=>c(()=>import("./lean-XBlWyCtg.js"), [])
        },
        {
            id: "less",
            name: "Less",
            import: ()=>c(()=>import("./less-BfCpw3nA.js"), [])
        },
        {
            id: "liquid",
            name: "Liquid",
            import: ()=>c(()=>import("./liquid-D3W5UaiH.js"), __vite__mapDeps([63,1,2,3,9]))
        },
        {
            id: "log",
            name: "Log file",
            import: ()=>c(()=>import("./log-Cc5clBb7.js"), [])
        },
        {
            id: "logo",
            name: "Logo",
            import: ()=>c(()=>import("./logo-IuBKFhSY.js"), [])
        },
        {
            id: "lua",
            name: "Lua",
            import: ()=>c(()=>import("./lua-CvWAzNxB.js"), __vite__mapDeps([37,24]))
        },
        {
            id: "luau",
            name: "Luau",
            import: ()=>c(()=>import("./luau-Du5NY7AG.js"), [])
        },
        {
            id: "make",
            name: "Makefile",
            aliases: [
                "makefile"
            ],
            import: ()=>c(()=>import("./make-Bvotw-X0.js"), [])
        },
        {
            id: "markdown",
            name: "Markdown",
            aliases: [
                "md"
            ],
            import: ()=>c(()=>import("./markdown-UIAJJxZW.js"), [])
        },
        {
            id: "marko",
            name: "Marko",
            import: ()=>c(()=>import("./marko-z0MBrx5-.js"), __vite__mapDeps([64,3,65,5,2]))
        },
        {
            id: "matlab",
            name: "MATLAB",
            import: ()=>c(()=>import("./matlab-D9-PGadD.js"), [])
        },
        {
            id: "mdc",
            name: "MDC",
            import: ()=>c(()=>import("./mdc-DB_EDNY_.js"), __vite__mapDeps([66,42,38,28,1,2,3]))
        },
        {
            id: "mdx",
            name: "MDX",
            import: ()=>c(()=>import("./mdx-sdHcTMYB.js"), [])
        },
        {
            id: "mermaid",
            name: "Mermaid",
            aliases: [
                "mmd"
            ],
            import: ()=>c(()=>import("./mermaid-Ci6OQyBP.js"), [])
        },
        {
            id: "mipsasm",
            name: "MIPS Assembly",
            aliases: [
                "mips"
            ],
            import: ()=>c(()=>import("./mipsasm-BC5c_5Pe.js"), [])
        },
        {
            id: "mojo",
            name: "Mojo",
            import: ()=>c(()=>import("./mojo-Tz6hzZYG.js"), [])
        },
        {
            id: "move",
            name: "Move",
            import: ()=>c(()=>import("./move-DB_GagMm.js"), [])
        },
        {
            id: "narrat",
            name: "Narrat Language",
            aliases: [
                "nar"
            ],
            import: ()=>c(()=>import("./narrat-DLbgOhZU.js"), [])
        },
        {
            id: "nextflow",
            name: "Nextflow",
            aliases: [
                "nf"
            ],
            import: ()=>c(()=>import("./nextflow-B0XVJmRM.js"), [])
        },
        {
            id: "nginx",
            name: "Nginx",
            import: ()=>c(()=>import("./nginx-D_VnBJ67.js"), __vite__mapDeps([67,37,24]))
        },
        {
            id: "nim",
            name: "Nim",
            import: ()=>c(()=>import("./nim-ZlGxZxc3.js"), __vite__mapDeps([68,24,1,2,3,7,8,23,42]))
        },
        {
            id: "nix",
            name: "Nix",
            import: ()=>c(()=>import("./nix-shcSOmrb.js"), [])
        },
        {
            id: "nushell",
            name: "nushell",
            aliases: [
                "nu"
            ],
            import: ()=>c(()=>import("./nushell-D4Tzg5kh.js"), [])
        },
        {
            id: "objective-c",
            name: "Objective-C",
            aliases: [
                "objc"
            ],
            import: ()=>c(()=>import("./objective-c-Deuh7S70.js"), [])
        },
        {
            id: "objective-cpp",
            name: "Objective-C++",
            import: ()=>c(()=>import("./objective-cpp-BUEGK8hf.js"), [])
        },
        {
            id: "ocaml",
            name: "OCaml",
            import: ()=>c(()=>import("./ocaml-BNioltXt.js"), [])
        },
        {
            id: "pascal",
            name: "Pascal",
            import: ()=>c(()=>import("./pascal-JqZropPD.js"), [])
        },
        {
            id: "perl",
            name: "Perl",
            import: ()=>c(()=>import("./perl-CHQXSrWU.js"), __vite__mapDeps([69,1,2,3,7,8,14]))
        },
        {
            id: "php",
            name: "PHP",
            import: ()=>c(()=>import("./php-B5ebYQev.js"), __vite__mapDeps([70,1,2,3,7,8,14,9]))
        },
        {
            id: "plsql",
            name: "PL/SQL",
            import: ()=>c(()=>import("./plsql-LKU2TuZ1.js"), [])
        },
        {
            id: "po",
            name: "Gettext PO",
            aliases: [
                "pot",
                "potx"
            ],
            import: ()=>c(()=>import("./po-BFLt1xDp.js"), [])
        },
        {
            id: "polar",
            name: "Polar",
            import: ()=>c(()=>import("./polar-DKykz6zU.js"), [])
        },
        {
            id: "postcss",
            name: "PostCSS",
            import: ()=>c(()=>import("./postcss-B3ZDOciz.js"), [])
        },
        {
            id: "powerquery",
            name: "PowerQuery",
            import: ()=>c(()=>import("./powerquery-CSHBycmS.js"), [])
        },
        {
            id: "powershell",
            name: "PowerShell",
            aliases: [
                "ps",
                "ps1"
            ],
            import: ()=>c(()=>import("./powershell-BIEUsx6d.js"), [])
        },
        {
            id: "prisma",
            name: "Prisma",
            import: ()=>c(()=>import("./prisma-B48N-Iqd.js"), [])
        },
        {
            id: "prolog",
            name: "Prolog",
            import: ()=>c(()=>import("./prolog-BY-TUvya.js"), [])
        },
        {
            id: "proto",
            name: "Protocol Buffer 3",
            aliases: [
                "protobuf"
            ],
            import: ()=>c(()=>import("./proto-zocC4JxJ.js"), [])
        },
        {
            id: "pug",
            name: "Pug",
            aliases: [
                "jade"
            ],
            import: ()=>c(()=>import("./pug-CM9l7STV.js"), __vite__mapDeps([71,2,3,1]))
        },
        {
            id: "puppet",
            name: "Puppet",
            import: ()=>c(()=>import("./puppet-Cza_XSSt.js"), [])
        },
        {
            id: "purescript",
            name: "PureScript",
            import: ()=>c(()=>import("./purescript-Bg-kzb6g.js"), [])
        },
        {
            id: "python",
            name: "Python",
            aliases: [
                "py"
            ],
            import: ()=>c(()=>import("./python-DhUJRlN_.js"), [])
        },
        {
            id: "qml",
            name: "QML",
            import: ()=>c(()=>import("./qml-D8XfuvdV.js"), __vite__mapDeps([72,2]))
        },
        {
            id: "qmldir",
            name: "QML Directory",
            import: ()=>c(()=>import("./qmldir-C8lEn-DE.js"), [])
        },
        {
            id: "qss",
            name: "Qt Style Sheets",
            import: ()=>c(()=>import("./qss-DhMKtDLN.js"), [])
        },
        {
            id: "r",
            name: "R",
            import: ()=>c(()=>import("./r-CwjWoCRV.js"), [])
        },
        {
            id: "racket",
            name: "Racket",
            import: ()=>c(()=>import("./racket-CzouJOBO.js"), [])
        },
        {
            id: "raku",
            name: "Raku",
            aliases: [
                "perl6"
            ],
            import: ()=>c(()=>import("./raku-B1bQXN8T.js"), [])
        },
        {
            id: "razor",
            name: "ASP.NET Razor",
            import: ()=>c(()=>import("./razor-CNLDkMZG.js"), __vite__mapDeps([73,1,2,3,74]))
        },
        {
            id: "reg",
            name: "Windows Registry Script",
            import: ()=>c(()=>import("./reg-5LuOXUq_.js"), [])
        },
        {
            id: "regexp",
            name: "RegExp",
            aliases: [
                "regex"
            ],
            import: ()=>c(()=>import("./regexp-DWJ3fJO_.js"), [])
        },
        {
            id: "rel",
            name: "Rel",
            import: ()=>c(()=>import("./rel-DJlmqQ1C.js"), [])
        },
        {
            id: "riscv",
            name: "RISC-V",
            import: ()=>c(()=>import("./riscv-QhoSD0DR.js"), [])
        },
        {
            id: "rst",
            name: "reStructuredText",
            import: ()=>c(()=>import("./rst-4NLicBqY.js"), __vite__mapDeps([75,28,1,2,3,21,22,23,24,14,18,26,38,76,32,33,7,8,34,11,35,36,37]))
        },
        {
            id: "ruby",
            name: "Ruby",
            aliases: [
                "rb"
            ],
            import: ()=>c(()=>import("./ruby-DeZ3UC14.js"), __vite__mapDeps([32,1,2,3,33,7,8,14,34,11,35,36,21,22,23,24,26,37,38]))
        },
        {
            id: "rust",
            name: "Rust",
            aliases: [
                "rs"
            ],
            import: ()=>c(()=>import("./rust-Be6lgOlo.js"), [])
        },
        {
            id: "sas",
            name: "SAS",
            import: ()=>c(()=>import("./sas-BmTFh92c.js"), __vite__mapDeps([77,14]))
        },
        {
            id: "sass",
            name: "Sass",
            import: ()=>c(()=>import("./sass-BJ4Li9vH.js"), [])
        },
        {
            id: "scala",
            name: "Scala",
            import: ()=>c(()=>import("./scala-DQVVAn-B.js"), [])
        },
        {
            id: "scheme",
            name: "Scheme",
            import: ()=>c(()=>import("./scheme-BJGe-b2p.js"), [])
        },
        {
            id: "scss",
            name: "SCSS",
            import: ()=>c(()=>import("./scss-C31hgJw-.js"), __vite__mapDeps([5,3]))
        },
        {
            id: "sdbl",
            name: "1C (Query)",
            aliases: [
                "1c-query"
            ],
            import: ()=>c(()=>import("./sdbl-BLhTXw86.js"), [])
        },
        {
            id: "shaderlab",
            name: "ShaderLab",
            aliases: [
                "shader"
            ],
            import: ()=>c(()=>import("./shaderlab-B7qAK45m.js"), __vite__mapDeps([78,79]))
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
            import: ()=>c(()=>import("./shellscript-atvbtKCR.js"), [])
        },
        {
            id: "shellsession",
            name: "Shell Session",
            aliases: [
                "console"
            ],
            import: ()=>c(()=>import("./shellsession-C_rIy8kc.js"), __vite__mapDeps([80,26]))
        },
        {
            id: "smalltalk",
            name: "Smalltalk",
            import: ()=>c(()=>import("./smalltalk-DkLiglaE.js"), [])
        },
        {
            id: "solidity",
            name: "Solidity",
            import: ()=>c(()=>import("./solidity-C1w2a3ep.js"), [])
        },
        {
            id: "soy",
            name: "Closure Templates",
            aliases: [
                "closure-templates"
            ],
            import: ()=>c(()=>import("./soy-C-lX7w71.js"), __vite__mapDeps([81,1,2,3]))
        },
        {
            id: "sparql",
            name: "SPARQL",
            import: ()=>c(()=>import("./sparql-bYkjHRlG.js"), __vite__mapDeps([82,83]))
        },
        {
            id: "splunk",
            name: "Splunk Query Language",
            aliases: [
                "spl"
            ],
            import: ()=>c(()=>import("./splunk-Cf8iN4DR.js"), [])
        },
        {
            id: "sql",
            name: "SQL",
            import: ()=>c(()=>import("./sql-COK4E0Yg.js"), [])
        },
        {
            id: "ssh-config",
            name: "SSH Config",
            import: ()=>c(()=>import("./ssh-config-BknIz3MU.js"), [])
        },
        {
            id: "stata",
            name: "Stata",
            import: ()=>c(()=>import("./stata-DorPZHa4.js"), __vite__mapDeps([84,14]))
        },
        {
            id: "stylus",
            name: "Stylus",
            aliases: [
                "styl"
            ],
            import: ()=>c(()=>import("./stylus-BeQkCIfX.js"), [])
        },
        {
            id: "svelte",
            name: "Svelte",
            import: ()=>c(()=>import("./svelte-MSaWC3Je.js"), __vite__mapDeps([85,2,11,3,12]))
        },
        {
            id: "swift",
            name: "Swift",
            import: ()=>c(()=>import("./swift-BSxZ-RaX.js"), [])
        },
        {
            id: "system-verilog",
            name: "SystemVerilog",
            import: ()=>c(()=>import("./system-verilog-C7L56vO4.js"), [])
        },
        {
            id: "systemd",
            name: "Systemd Units",
            import: ()=>c(()=>import("./systemd-CUnW07Te.js"), [])
        },
        {
            id: "talonscript",
            name: "TalonScript",
            aliases: [
                "talon"
            ],
            import: ()=>c(()=>import("./talonscript-C1XDQQGZ.js"), [])
        },
        {
            id: "tasl",
            name: "Tasl",
            import: ()=>c(()=>import("./tasl-CQjiPCtT.js"), [])
        },
        {
            id: "tcl",
            name: "Tcl",
            import: ()=>c(()=>import("./tcl-DQ1-QYvQ.js"), [])
        },
        {
            id: "templ",
            name: "Templ",
            import: ()=>c(()=>import("./templ-dwX3ZSMB.js"), __vite__mapDeps([86,87,2,3]))
        },
        {
            id: "terraform",
            name: "Terraform",
            aliases: [
                "tf",
                "tfvars"
            ],
            import: ()=>c(()=>import("./terraform-BbSNqyBO.js"), [])
        },
        {
            id: "tex",
            name: "TeX",
            import: ()=>c(()=>import("./tex-rYs2v40G.js"), __vite__mapDeps([62,60]))
        },
        {
            id: "toml",
            name: "TOML",
            import: ()=>c(()=>import("./toml-CB2ApiWb.js"), [])
        },
        {
            id: "ts-tags",
            name: "TypeScript with Tags",
            aliases: [
                "lit"
            ],
            import: ()=>c(()=>import("./ts-tags-CipyTH0X.js"), __vite__mapDeps([88,11,3,2,23,24,1,14,7,8]))
        },
        {
            id: "tsv",
            name: "TSV",
            import: ()=>c(()=>import("./tsv-B_m7g4N7.js"), [])
        },
        {
            id: "tsx",
            name: "TSX",
            import: ()=>c(()=>import("./tsx-B6W0miNI.js"), [])
        },
        {
            id: "turtle",
            name: "Turtle",
            import: ()=>c(()=>import("./turtle-BMR_PYu6.js"), [])
        },
        {
            id: "twig",
            name: "Twig",
            import: ()=>c(()=>import("./twig-NC5TFiHP.js"), __vite__mapDeps([89,3,2,5,70,1,7,8,14,9,18,32,33,34,11,35,36,21,22,23,24,26,37,38]))
        },
        {
            id: "typescript",
            name: "TypeScript",
            aliases: [
                "ts"
            ],
            import: ()=>c(()=>import("./typescript-Dj6nwHGl.js"), [])
        },
        {
            id: "typespec",
            name: "TypeSpec",
            aliases: [
                "tsp"
            ],
            import: ()=>c(()=>import("./typespec-BpWG_bgh.js"), [])
        },
        {
            id: "typst",
            name: "Typst",
            aliases: [
                "typ"
            ],
            import: ()=>c(()=>import("./typst-BVUVsWT6.js"), [])
        },
        {
            id: "v",
            name: "V",
            import: ()=>c(()=>import("./v-CAQ2eGtk.js"), [])
        },
        {
            id: "vala",
            name: "Vala",
            import: ()=>c(()=>import("./vala-BFOHcciG.js"), [])
        },
        {
            id: "vb",
            name: "Visual Basic",
            aliases: [
                "cmd"
            ],
            import: ()=>c(()=>import("./vb-CdO5JTpU.js"), [])
        },
        {
            id: "verilog",
            name: "Verilog",
            import: ()=>c(()=>import("./verilog-CJaU5se_.js"), [])
        },
        {
            id: "vhdl",
            name: "VHDL",
            import: ()=>c(()=>import("./vhdl-DYoNaHQp.js"), [])
        },
        {
            id: "viml",
            name: "Vim Script",
            aliases: [
                "vim",
                "vimscript"
            ],
            import: ()=>c(()=>import("./viml-m4uW47V2.js"), [])
        },
        {
            id: "vue",
            name: "Vue",
            import: ()=>c(()=>import("./vue-BuYVFjOK.js"), __vite__mapDeps([90,1,2,3,11,9,28]))
        },
        {
            id: "vue-html",
            name: "Vue HTML",
            import: ()=>c(()=>import("./vue-html-xdeiXROB.js"), __vite__mapDeps([91,90,1,2,3,11,9,28]))
        },
        {
            id: "vyper",
            name: "Vyper",
            aliases: [
                "vy"
            ],
            import: ()=>c(()=>import("./vyper-nyqBNV6O.js"), [])
        },
        {
            id: "wasm",
            name: "WebAssembly",
            import: ()=>c(()=>import("./wasm-C6j12Q_x.js"), [])
        },
        {
            id: "wenyan",
            name: "Wenyan",
            aliases: [
                "文言"
            ],
            import: ()=>c(()=>import("./wenyan-7A4Fjokl.js"), [])
        },
        {
            id: "wgsl",
            name: "WGSL",
            import: ()=>c(()=>import("./wgsl-CB0Krxn9.js"), [])
        },
        {
            id: "wikitext",
            name: "Wikitext",
            aliases: [
                "mediawiki",
                "wiki"
            ],
            import: ()=>c(()=>import("./wikitext-DCE3LsBG.js"), [])
        },
        {
            id: "wolfram",
            name: "Wolfram",
            aliases: [
                "wl"
            ],
            import: ()=>c(()=>import("./wolfram-C3FkfJm5.js"), [])
        },
        {
            id: "xml",
            name: "XML",
            import: ()=>c(()=>import("./xml-e3z08dGr.js"), __vite__mapDeps([7,8]))
        },
        {
            id: "xsl",
            name: "XSL",
            import: ()=>c(()=>import("./xsl-Dd0NUgwM.js"), __vite__mapDeps([92,7,8]))
        },
        {
            id: "yaml",
            name: "YAML",
            aliases: [
                "yml"
            ],
            import: ()=>c(()=>import("./yaml-CVw76BM1.js"), [])
        },
        {
            id: "zenscript",
            name: "ZenScript",
            import: ()=>c(()=>import("./zenscript-HnGAYVZD.js"), [])
        },
        {
            id: "zig",
            name: "Zig",
            import: ()=>c(()=>import("./zig-BVz_zdnA.js"), [])
        }
    ], es = Object.fromEntries(cr.map((t)=>[
            t.id,
            t.import
        ])), ts = Object.fromEntries(cr.flatMap((t)=>t.aliases?.map((e)=>[
                e,
                t.import
            ]) || [])), ns = {
        ...es,
        ...ts
    }, rs = [
        {
            id: "andromeeda",
            displayName: "Andromeeda",
            type: "dark",
            import: ()=>c(()=>import("./andromeeda-C3khCPGq.js"), [])
        },
        {
            id: "aurora-x",
            displayName: "Aurora X",
            type: "dark",
            import: ()=>c(()=>import("./aurora-x-D-2ljcwZ.js"), [])
        },
        {
            id: "ayu-dark",
            displayName: "Ayu Dark",
            type: "dark",
            import: ()=>c(()=>import("./ayu-dark-Cv9koXgw.js"), [])
        },
        {
            id: "catppuccin-frappe",
            displayName: "Catppuccin Frappé",
            type: "dark",
            import: ()=>c(()=>import("./catppuccin-frappe-CD_QflpE.js"), [])
        },
        {
            id: "catppuccin-latte",
            displayName: "Catppuccin Latte",
            type: "light",
            import: ()=>c(()=>import("./catppuccin-latte-DRW-0cLl.js"), [])
        },
        {
            id: "catppuccin-macchiato",
            displayName: "Catppuccin Macchiato",
            type: "dark",
            import: ()=>c(()=>import("./catppuccin-macchiato-C-_shW-Y.js"), [])
        },
        {
            id: "catppuccin-mocha",
            displayName: "Catppuccin Mocha",
            type: "dark",
            import: ()=>c(()=>import("./catppuccin-mocha-LGGdnPYs.js"), [])
        },
        {
            id: "dark-plus",
            displayName: "Dark Plus",
            type: "dark",
            import: ()=>c(()=>import("./dark-plus-C3mMm8J8.js"), [])
        },
        {
            id: "dracula",
            displayName: "Dracula Theme",
            type: "dark",
            import: ()=>c(()=>import("./dracula-BzJJZx-M.js"), [])
        },
        {
            id: "dracula-soft",
            displayName: "Dracula Theme Soft",
            type: "dark",
            import: ()=>c(()=>import("./dracula-soft-BXkSAIEj.js"), [])
        },
        {
            id: "everforest-dark",
            displayName: "Everforest Dark",
            type: "dark",
            import: ()=>c(()=>import("./everforest-dark-BgDCqdQA.js"), [])
        },
        {
            id: "everforest-light",
            displayName: "Everforest Light",
            type: "light",
            import: ()=>c(()=>import("./everforest-light-C8M2exoo.js"), [])
        },
        {
            id: "github-dark",
            displayName: "GitHub Dark",
            type: "dark",
            import: ()=>c(()=>import("./github-dark-DHJKELXO.js"), [])
        },
        {
            id: "github-dark-default",
            displayName: "GitHub Dark Default",
            type: "dark",
            import: ()=>c(()=>import("./github-dark-default-Cuk6v7N8.js"), [])
        },
        {
            id: "github-dark-dimmed",
            displayName: "GitHub Dark Dimmed",
            type: "dark",
            import: ()=>c(()=>import("./github-dark-dimmed-DH5Ifo-i.js"), [])
        },
        {
            id: "github-dark-high-contrast",
            displayName: "GitHub Dark High Contrast",
            type: "dark",
            import: ()=>c(()=>import("./github-dark-high-contrast-E3gJ1_iC.js"), [])
        },
        {
            id: "github-light",
            displayName: "GitHub Light",
            type: "light",
            import: ()=>c(()=>import("./github-light-DAi9KRSo.js"), [])
        },
        {
            id: "github-light-default",
            displayName: "GitHub Light Default",
            type: "light",
            import: ()=>c(()=>import("./github-light-default-D7oLnXFd.js"), [])
        },
        {
            id: "github-light-high-contrast",
            displayName: "GitHub Light High Contrast",
            type: "light",
            import: ()=>c(()=>import("./github-light-high-contrast-BfjtVDDH.js"), [])
        },
        {
            id: "houston",
            displayName: "Houston",
            type: "dark",
            import: ()=>c(()=>import("./houston-DnULxvSX.js"), [])
        },
        {
            id: "kanagawa-dragon",
            displayName: "Kanagawa Dragon",
            type: "dark",
            import: ()=>c(()=>import("./kanagawa-dragon-CkXjmgJE.js"), [])
        },
        {
            id: "kanagawa-lotus",
            displayName: "Kanagawa Lotus",
            type: "light",
            import: ()=>c(()=>import("./kanagawa-lotus-CfQXZHmo.js"), [])
        },
        {
            id: "kanagawa-wave",
            displayName: "Kanagawa Wave",
            type: "dark",
            import: ()=>c(()=>import("./kanagawa-wave-DWedfzmr.js"), [])
        },
        {
            id: "laserwave",
            displayName: "LaserWave",
            type: "dark",
            import: ()=>c(()=>import("./laserwave-DUszq2jm.js"), [])
        },
        {
            id: "light-plus",
            displayName: "Light Plus",
            type: "light",
            import: ()=>c(()=>import("./light-plus-B7mTdjB0.js"), [])
        },
        {
            id: "material-theme",
            displayName: "Material Theme",
            type: "dark",
            import: ()=>c(()=>import("./material-theme-D5KoaKCx.js"), [])
        },
        {
            id: "material-theme-darker",
            displayName: "Material Theme Darker",
            type: "dark",
            import: ()=>c(()=>import("./material-theme-darker-BfHTSMKl.js"), [])
        },
        {
            id: "material-theme-lighter",
            displayName: "Material Theme Lighter",
            type: "light",
            import: ()=>c(()=>import("./material-theme-lighter-B0m2ddpp.js"), [])
        },
        {
            id: "material-theme-ocean",
            displayName: "Material Theme Ocean",
            type: "dark",
            import: ()=>c(()=>import("./material-theme-ocean-CyktbL80.js"), [])
        },
        {
            id: "material-theme-palenight",
            displayName: "Material Theme Palenight",
            type: "dark",
            import: ()=>c(()=>import("./material-theme-palenight-Csfq5Kiy.js"), [])
        },
        {
            id: "min-dark",
            displayName: "Min Dark",
            type: "dark",
            import: ()=>c(()=>import("./min-dark-CafNBF8u.js"), [])
        },
        {
            id: "min-light",
            displayName: "Min Light",
            type: "light",
            import: ()=>c(()=>import("./min-light-CTRr51gU.js"), [])
        },
        {
            id: "monokai",
            displayName: "Monokai",
            type: "dark",
            import: ()=>c(()=>import("./monokai-D4h5O-jR.js"), [])
        },
        {
            id: "night-owl",
            displayName: "Night Owl",
            type: "dark",
            import: ()=>c(()=>import("./night-owl-C39BiMTA.js"), [])
        },
        {
            id: "nord",
            displayName: "Nord",
            type: "dark",
            import: ()=>c(()=>import("./nord-Ddv68eIx.js"), [])
        },
        {
            id: "one-dark-pro",
            displayName: "One Dark Pro",
            type: "dark",
            import: ()=>c(()=>import("./one-dark-pro-GBQ2dnAY.js"), [])
        },
        {
            id: "one-light",
            displayName: "One Light",
            type: "light",
            import: ()=>c(()=>import("./one-light-PoHY5YXO.js"), [])
        },
        {
            id: "plastic",
            displayName: "Plastic",
            type: "dark",
            import: ()=>c(()=>import("./plastic-3e1v2bzS.js"), [])
        },
        {
            id: "poimandres",
            displayName: "Poimandres",
            type: "dark",
            import: ()=>c(()=>import("./poimandres-CS3Unz2-.js"), [])
        },
        {
            id: "red",
            displayName: "Red",
            type: "dark",
            import: ()=>c(()=>import("./red-bN70gL4F.js"), [])
        },
        {
            id: "rose-pine",
            displayName: "Rosé Pine",
            type: "dark",
            import: ()=>c(()=>import("./rose-pine-CmCqftbK.js"), [])
        },
        {
            id: "rose-pine-dawn",
            displayName: "Rosé Pine Dawn",
            type: "light",
            import: ()=>c(()=>import("./rose-pine-dawn-Ds-gbosJ.js"), [])
        },
        {
            id: "rose-pine-moon",
            displayName: "Rosé Pine Moon",
            type: "dark",
            import: ()=>c(()=>import("./rose-pine-moon-CjDtw9vr.js"), [])
        },
        {
            id: "slack-dark",
            displayName: "Slack Dark",
            type: "dark",
            import: ()=>c(()=>import("./slack-dark-BthQWCQV.js"), [])
        },
        {
            id: "slack-ochin",
            displayName: "Slack Ochin",
            type: "light",
            import: ()=>c(()=>import("./slack-ochin-DqwNpetd.js"), [])
        },
        {
            id: "snazzy-light",
            displayName: "Snazzy Light",
            type: "light",
            import: ()=>c(()=>import("./snazzy-light-Bw305WKR.js"), [])
        },
        {
            id: "solarized-dark",
            displayName: "Solarized Dark",
            type: "dark",
            import: ()=>c(()=>import("./solarized-dark-DXbdFlpD.js"), [])
        },
        {
            id: "solarized-light",
            displayName: "Solarized Light",
            type: "light",
            import: ()=>c(()=>import("./solarized-light-L9t79GZl.js"), [])
        },
        {
            id: "synthwave-84",
            displayName: "Synthwave '84",
            type: "dark",
            import: ()=>c(()=>import("./synthwave-84-CbfX1IO0.js"), [])
        },
        {
            id: "tokyo-night",
            displayName: "Tokyo Night",
            type: "dark",
            import: ()=>c(()=>import("./tokyo-night-DBQeEorK.js"), [])
        },
        {
            id: "vesper",
            displayName: "Vesper",
            type: "dark",
            import: ()=>c(()=>import("./vesper-BEBZ7ncR.js"), [])
        },
        {
            id: "vitesse-black",
            displayName: "Vitesse Black",
            type: "dark",
            import: ()=>c(()=>import("./vitesse-black-Bkuqu6BP.js"), [])
        },
        {
            id: "vitesse-dark",
            displayName: "Vitesse Dark",
            type: "dark",
            import: ()=>c(()=>import("./vitesse-dark-D0r3Knsf.js"), [])
        },
        {
            id: "vitesse-light",
            displayName: "Vitesse Light",
            type: "light",
            import: ()=>c(()=>import("./vitesse-light-CVO1_9PV.js"), [])
        }
    ], is = Object.fromEntries(rs.map((t)=>[
            t.id,
            t.import
        ]));
    let ce = class extends Error {
        constructor(e){
            super(e), this.name = "ShikiError";
        }
    }, sn = class extends Error {
        constructor(e){
            super(e), this.name = "ShikiError";
        }
    };
    function os() {
        return 2147483648;
    }
    function ss() {
        return typeof performance < "u" ? performance.now() : Date.now();
    }
    const as = (t, e)=>t + (e - t % e) % e;
    async function ls(t) {
        let e, n;
        const r = {};
        function i(f) {
            n = f, r.HEAPU8 = new Uint8Array(f), r.HEAPU32 = new Uint32Array(f);
        }
        function o(f, m, w) {
            r.HEAPU8.copyWithin(f, m, m + w);
        }
        function s(f) {
            try {
                return e.grow(f - n.byteLength + 65535 >>> 16), i(e.buffer), 1;
            } catch  {}
        }
        function a(f) {
            const m = r.HEAPU8.length;
            f = f >>> 0;
            const w = os();
            if (f > w) return !1;
            for(let g = 1; g <= 4; g *= 2){
                let E = m * (1 + .2 / g);
                E = Math.min(E, f + 100663296);
                const p = Math.min(w, as(Math.max(f, E), 65536));
                if (s(p)) return !0;
            }
            return !1;
        }
        const l = typeof TextDecoder < "u" ? new TextDecoder("utf8") : void 0;
        function u(f, m, w = 1024) {
            const g = m + w;
            let E = m;
            for(; f[E] && !(E >= g);)++E;
            if (E - m > 16 && f.buffer && l) return l.decode(f.subarray(m, E));
            let p = "";
            for(; m < E;){
                let b = f[m++];
                if (!(b & 128)) {
                    p += String.fromCharCode(b);
                    continue;
                }
                const v = f[m++] & 63;
                if ((b & 224) === 192) {
                    p += String.fromCharCode((b & 31) << 6 | v);
                    continue;
                }
                const T = f[m++] & 63;
                if ((b & 240) === 224 ? b = (b & 15) << 12 | v << 6 | T : b = (b & 7) << 18 | v << 12 | T << 6 | f[m++] & 63, b < 65536) p += String.fromCharCode(b);
                else {
                    const O = b - 65536;
                    p += String.fromCharCode(55296 | O >> 10, 56320 | O & 1023);
                }
            }
            return p;
        }
        function h(f, m) {
            return f ? u(r.HEAPU8, f, m) : "";
        }
        const d = {
            emscripten_get_now: ss,
            emscripten_memcpy_big: o,
            emscripten_resize_heap: a,
            fd_write: ()=>0
        };
        async function _() {
            const m = await t({
                env: d,
                wasi_snapshot_preview1: d
            });
            e = m.memory, i(e.buffer), Object.assign(r, m), r.UTF8ToString = h;
        }
        return await _(), r;
    }
    var cs = Object.defineProperty, us = (t, e, n)=>e in t ? cs(t, e, {
            enumerable: !0,
            configurable: !0,
            writable: !0,
            value: n
        }) : t[e] = n, $ = (t, e, n)=>(us(t, typeof e != "symbol" ? e + "" : e, n), n);
    let B = null;
    function ds(t) {
        throw new sn(t.UTF8ToString(t.getLastOnigError()));
    }
    class Rt {
        constructor(e){
            $(this, "utf16Length"), $(this, "utf8Length"), $(this, "utf16Value"), $(this, "utf8Value"), $(this, "utf16OffsetToUtf8"), $(this, "utf8OffsetToUtf16");
            const n = e.length, r = Rt._utf8ByteLength(e), i = r !== n, o = i ? new Uint32Array(n + 1) : null;
            i && (o[n] = r);
            const s = i ? new Uint32Array(r + 1) : null;
            i && (s[r] = n);
            const a = new Uint8Array(r);
            let l = 0;
            for(let u = 0; u < n; u++){
                const h = e.charCodeAt(u);
                let d = h, _ = !1;
                if (h >= 55296 && h <= 56319 && u + 1 < n) {
                    const f = e.charCodeAt(u + 1);
                    f >= 56320 && f <= 57343 && (d = (h - 55296 << 10) + 65536 | f - 56320, _ = !0);
                }
                i && (o[u] = l, _ && (o[u + 1] = l), d <= 127 ? s[l + 0] = u : d <= 2047 ? (s[l + 0] = u, s[l + 1] = u) : d <= 65535 ? (s[l + 0] = u, s[l + 1] = u, s[l + 2] = u) : (s[l + 0] = u, s[l + 1] = u, s[l + 2] = u, s[l + 3] = u)), d <= 127 ? a[l++] = d : d <= 2047 ? (a[l++] = 192 | (d & 1984) >>> 6, a[l++] = 128 | (d & 63) >>> 0) : d <= 65535 ? (a[l++] = 224 | (d & 61440) >>> 12, a[l++] = 128 | (d & 4032) >>> 6, a[l++] = 128 | (d & 63) >>> 0) : (a[l++] = 240 | (d & 1835008) >>> 18, a[l++] = 128 | (d & 258048) >>> 12, a[l++] = 128 | (d & 4032) >>> 6, a[l++] = 128 | (d & 63) >>> 0), _ && u++;
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
    const te = class {
        constructor(t){
            if ($(this, "id", ++te.LAST_ID), $(this, "_onigBinding"), $(this, "content"), $(this, "utf16Length"), $(this, "utf8Length"), $(this, "utf16OffsetToUtf8"), $(this, "utf8OffsetToUtf16"), $(this, "ptr"), !B) throw new sn("Must invoke loadWasm first.");
            this._onigBinding = B, this.content = t;
            const e = new Rt(t);
            this.utf16Length = e.utf16Length, this.utf8Length = e.utf8Length, this.utf16OffsetToUtf8 = e.utf16OffsetToUtf8, this.utf8OffsetToUtf16 = e.utf8OffsetToUtf16, this.utf8Length < 1e4 && !te._sharedPtrInUse ? (te._sharedPtr || (te._sharedPtr = B.omalloc(1e4)), te._sharedPtrInUse = !0, B.HEAPU8.set(e.utf8Value, te._sharedPtr), this.ptr = te._sharedPtr) : this.ptr = e.createString(B);
        }
        convertUtf8OffsetToUtf16(t) {
            return this.utf8OffsetToUtf16 ? t < 0 ? 0 : t > this.utf8Length ? this.utf16Length : this.utf8OffsetToUtf16[t] : t;
        }
        convertUtf16OffsetToUtf8(t) {
            return this.utf16OffsetToUtf8 ? t < 0 ? 0 : t > this.utf16Length ? this.utf8Length : this.utf16OffsetToUtf8[t] : t;
        }
        dispose() {
            this.ptr === te._sharedPtr ? te._sharedPtrInUse = !1 : this._onigBinding.ofree(this.ptr);
        }
    };
    let Fe = te;
    $(Fe, "LAST_ID", 0);
    $(Fe, "_sharedPtr", 0);
    $(Fe, "_sharedPtrInUse", !1);
    class hs {
        constructor(e){
            if ($(this, "_onigBinding"), $(this, "_ptr"), !B) throw new sn("Must invoke loadWasm first.");
            const n = [], r = [];
            for(let a = 0, l = e.length; a < l; a++){
                const u = new Rt(e[a]);
                n[a] = u.createString(B), r[a] = u.utf8Length;
            }
            const i = B.omalloc(4 * e.length);
            B.HEAPU32.set(n, i / 4);
            const o = B.omalloc(4 * e.length);
            B.HEAPU32.set(r, o / 4);
            const s = B.createOnigScanner(i, o, e.length);
            for(let a = 0, l = e.length; a < l; a++)B.ofree(n[a]);
            B.ofree(o), B.ofree(i), s === 0 && ds(B), this._onigBinding = B, this._ptr = s;
        }
        dispose() {
            this._onigBinding.freeOnigScanner(this._ptr);
        }
        findNextMatchSync(e, n, r) {
            let i = 0;
            if (typeof r == "number" && (i = r), typeof e == "string") {
                e = new Fe(e);
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
            const u = a[l++], h = a[l++], d = [];
            for(let _ = 0; _ < h; _++){
                const f = e.convertUtf8OffsetToUtf16(a[l++]), m = e.convertUtf8OffsetToUtf16(a[l++]);
                d[_] = {
                    start: f,
                    end: m,
                    length: m - f
                };
            }
            return {
                index: u,
                captureIndices: d
            };
        }
    }
    function ms(t) {
        return typeof t.instantiator == "function";
    }
    function fs(t) {
        return typeof t.default == "function";
    }
    function ps(t) {
        return typeof t.data < "u";
    }
    function _s(t) {
        return typeof Response < "u" && t instanceof Response;
    }
    function gs(t) {
        return typeof ArrayBuffer < "u" && (t instanceof ArrayBuffer || ArrayBuffer.isView(t)) || typeof Buffer < "u" && Buffer.isBuffer?.(t) || typeof SharedArrayBuffer < "u" && t instanceof SharedArrayBuffer || typeof Uint32Array < "u" && t instanceof Uint32Array;
    }
    let Ze;
    function ys(t) {
        if (Ze) return Ze;
        async function e() {
            B = await ls(async (n)=>{
                let r = t;
                return r = await r, typeof r == "function" && (r = await r(n)), typeof r == "function" && (r = await r(n)), ms(r) ? r = await r.instantiator(n) : fs(r) ? r = await r.default(n) : (ps(r) && (r = r.data), _s(r) ? typeof WebAssembly.instantiateStreaming == "function" ? r = await bs(r)(n) : r = await Es(r)(n) : gs(r) ? r = await Nt(r)(n) : r instanceof WebAssembly.Module ? r = await Nt(r)(n) : "default" in r && r.default instanceof WebAssembly.Module && (r = await Nt(r.default)(n))), "instance" in r && (r = r.instance), "exports" in r && (r = r.exports), r;
            });
        }
        return Ze = e(), Ze;
    }
    function Nt(t) {
        return (e)=>WebAssembly.instantiate(t, e);
    }
    function bs(t) {
        return (e)=>WebAssembly.instantiateStreaming(t, e);
    }
    function Es(t) {
        return async (e)=>{
            const n = await t.arrayBuffer();
            return WebAssembly.instantiate(n, e);
        };
    }
    let ws;
    function vs() {
        return ws;
    }
    async function ur(t) {
        return t && await ys(t), {
            createScanner (e) {
                return new hs(e.map((n)=>typeof n == "string" ? n : n.source));
            },
            createString (e) {
                return new Fe(e);
            }
        };
    }
    function Ss(t) {
        return an(t);
    }
    function an(t) {
        return Array.isArray(t) ? As(t) : t instanceof RegExp ? t : typeof t == "object" ? Ts(t) : t;
    }
    function As(t) {
        let e = [];
        for(let n = 0, r = t.length; n < r; n++)e[n] = an(t[n]);
        return e;
    }
    function Ts(t) {
        let e = {};
        for(let n in t)e[n] = an(t[n]);
        return e;
    }
    function dr(t, ...e) {
        return e.forEach((n)=>{
            for(let r in n)t[r] = n[r];
        }), t;
    }
    function hr(t) {
        const e = ~t.lastIndexOf("/") || ~t.lastIndexOf("\\");
        return e === 0 ? t : ~e === t.length - 1 ? hr(t.substring(0, t.length - 1)) : t.substr(~e + 1);
    }
    var Vt = /\$(\d+)|\${(\d+):\/(downcase|upcase)}/g, et = class {
        static hasCaptures(t) {
            return t === null ? !1 : (Vt.lastIndex = 0, Vt.test(t));
        }
        static replaceCaptures(t, e, n) {
            return t.replace(Vt, (r, i, o, s)=>{
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
    function mr(t, e) {
        return t < e ? -1 : t > e ? 1 : 0;
    }
    function fr(t, e) {
        if (t === null && e === null) return 0;
        if (!t) return -1;
        if (!e) return 1;
        let n = t.length, r = e.length;
        if (n === r) {
            for(let i = 0; i < n; i++){
                let o = mr(t[i], e[i]);
                if (o !== 0) return o;
            }
            return 0;
        }
        return n - r;
    }
    function kn(t) {
        return !!(/^#[0-9a-f]{6}$/i.test(t) || /^#[0-9a-f]{8}$/i.test(t) || /^#[0-9a-f]{3}$/i.test(t) || /^#[0-9a-f]{4}$/i.test(t));
    }
    function pr(t) {
        return t.replace(/[\-\\\{\}\*\+\?\|\^\$\.\,\[\]\(\)\#\s]/g, "\\$&");
    }
    var _r = class {
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
            return this.createFromParsedTheme(Ps(t), e);
        }
        static createFromParsedTheme(t, e) {
            return Cs(t, e);
        }
        _cachedMatchRoot = new _r((t)=>this._root.match(t));
        getColorMap() {
            return this._colorMap.getColorMap();
        }
        getDefaults() {
            return this._defaults;
        }
        match(t) {
            if (t === null) return this._defaults;
            const e = t.scopeName, r = this._cachedMatchRoot.get(e).find((i)=>Rs(t.parent, i.parentScopes));
            return r ? new gr(r.fontStyle, r.foreground, r.background) : null;
        }
    }, Mt = class st {
        constructor(e, n){
            this.parent = e, this.scopeName = n;
        }
        static push(e, n) {
            for (const r of n)e = new st(e, r);
            return e;
        }
        static from(...e) {
            let n = null;
            for(let r = 0; r < e.length; r++)n = new st(n, e[r]);
            return n;
        }
        push(e) {
            return new st(this, e);
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
    function Rs(t, e) {
        if (e.length === 0) return !0;
        for(let n = 0; n < e.length; n++){
            let r = e[n], i = !1;
            if (r === ">") {
                if (n === e.length - 1) return !1;
                r = e[++n], i = !0;
            }
            for(; t && !Ls(t.scopeName, r);){
                if (i) return !1;
                t = t.parent;
            }
            if (!t) return !1;
            t = t.parent;
        }
        return !0;
    }
    function Ls(t, e) {
        return e === t || t.startsWith(e) && t[e.length] === ".";
    }
    var gr = class {
        constructor(t, e, n){
            this.fontStyle = t, this.foregroundId = e, this.backgroundId = n;
        }
    };
    function Ps(t) {
        if (!t) return [];
        if (!t.settings || !Array.isArray(t.settings)) return [];
        let e = t.settings, n = [], r = 0;
        for(let i = 0, o = e.length; i < o; i++){
            let s = e[i];
            if (!s.settings) continue;
            let a;
            if (typeof s.scope == "string") {
                let d = s.scope;
                d = d.replace(/^[,]+/, ""), d = d.replace(/[,]+$/, ""), a = d.split(",");
            } else Array.isArray(s.scope) ? a = s.scope : a = [
                ""
            ];
            let l = -1;
            if (typeof s.settings.fontStyle == "string") {
                l = 0;
                let d = s.settings.fontStyle.split(" ");
                for(let _ = 0, f = d.length; _ < f; _++)switch(d[_]){
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
            let u = null;
            typeof s.settings.foreground == "string" && kn(s.settings.foreground) && (u = s.settings.foreground);
            let h = null;
            typeof s.settings.background == "string" && kn(s.settings.background) && (h = s.settings.background);
            for(let d = 0, _ = a.length; d < _; d++){
                let m = a[d].trim().split(" "), w = m[m.length - 1], g = null;
                m.length > 1 && (g = m.slice(0, m.length - 1), g.reverse()), n[r++] = new ks(w, g, i, l, u, h);
            }
        }
        return n;
    }
    var ks = class {
        constructor(t, e, n, r, i, o){
            this.scope = t, this.parentScopes = e, this.index = n, this.fontStyle = r, this.foreground = i, this.background = o;
        }
    }, ae = ((t)=>(t[t.NotSet = -1] = "NotSet", t[t.None = 0] = "None", t[t.Italic = 1] = "Italic", t[t.Bold = 2] = "Bold", t[t.Underline = 4] = "Underline", t[t.Strikethrough = 8] = "Strikethrough", t))(ae || {});
    function Cs(t, e) {
        t.sort((l, u)=>{
            let h = mr(l.scope, u.scope);
            return h !== 0 || (h = fr(l.parentScopes, u.parentScopes), h !== 0) ? h : l.index - u.index;
        });
        let n = 0, r = "#000000", i = "#ffffff";
        for(; t.length >= 1 && t[0].scope === "";){
            let l = t.shift();
            l.fontStyle !== -1 && (n = l.fontStyle), l.foreground !== null && (r = l.foreground), l.background !== null && (i = l.background);
        }
        let o = new Is(e), s = new gr(n, o.getId(r), o.getId(i)), a = new xs(new zt(0, null, -1, 0, 0), []);
        for(let l = 0, u = t.length; l < u; l++){
            let h = t[l];
            a.insert(0, h.scope, h.parentScopes, h.fontStyle, o.getId(h.foreground), o.getId(h.background));
        }
        return new mt(o, s, a);
    }
    var Is = class {
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
    }, Os = Object.freeze([]), zt = class yr {
        scopeDepth;
        parentScopes;
        fontStyle;
        foreground;
        background;
        constructor(e, n, r, i, o){
            this.scopeDepth = e, this.parentScopes = n || Os, this.fontStyle = r, this.foreground = i, this.background = o;
        }
        clone() {
            return new yr(this.scopeDepth, this.parentScopes, this.fontStyle, this.foreground, this.background);
        }
        static cloneArr(e) {
            let n = [];
            for(let r = 0, i = e.length; r < i; r++)n[r] = e[r].clone();
            return n;
        }
        acceptOverwrite(e, n, r, i) {
            this.scopeDepth > e ? console.log("how did this happen?") : this.scopeDepth = e, n !== -1 && (this.fontStyle = n), r !== 0 && (this.foreground = r), i !== 0 && (this.background = i);
        }
    }, xs = class qt {
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
            return n.sort(qt._cmpBySpecificity), n;
        }
        insert(e, n, r, i, o, s) {
            if (n === "") {
                this._doInsertHere(e, r, i, o, s);
                return;
            }
            let a = n.indexOf("."), l, u;
            a === -1 ? (l = n, u = "") : (l = n.substring(0, a), u = n.substring(a + 1));
            let h;
            this._children.hasOwnProperty(l) ? h = this._children[l] : (h = new qt(this._mainRule.clone(), zt.cloneArr(this._rulesWithParentScopes)), this._children[l] = h), h.insert(e + 1, u, r, i, o, s);
        }
        _doInsertHere(e, n, r, i, o) {
            if (n === null) {
                this._mainRule.acceptOverwrite(e, r, i, o);
                return;
            }
            for(let s = 0, a = this._rulesWithParentScopes.length; s < a; s++){
                let l = this._rulesWithParentScopes[s];
                if (fr(l.parentScopes, n) === 0) {
                    l.acceptOverwrite(e, r, i, o);
                    return;
                }
            }
            r === -1 && (r = this._mainRule.fontStyle), i === 0 && (i = this._mainRule.foreground), o === 0 && (o = this._mainRule.background), this._rulesWithParentScopes.push(new zt(e, n, r, i, o));
        }
    }, Re = class Y {
        static toBinaryStr(e) {
            return e.toString(2).padStart(32, "0");
        }
        static print(e) {
            const n = Y.getLanguageId(e), r = Y.getTokenType(e), i = Y.getFontStyle(e), o = Y.getForeground(e), s = Y.getBackground(e);
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
            let l = Y.getLanguageId(e), u = Y.getTokenType(e), h = Y.containsBalancedBrackets(e) ? 1 : 0, d = Y.getFontStyle(e), _ = Y.getForeground(e), f = Y.getBackground(e);
            return n !== 0 && (l = n), r !== 8 && (u = r), i !== null && (h = i ? 1 : 0), o !== -1 && (d = o), s !== 0 && (_ = s), a !== 0 && (f = a), (l << 0 | u << 8 | h << 10 | d << 11 | _ << 15 | f << 24) >>> 0;
        }
    };
    function ft(t, e) {
        const n = [], r = Ds(t);
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
            let u = s();
            if (n.push({
                matcher: u,
                priority: l
            }), i !== ",") break;
            i = r.next();
        }
        return n;
        function o() {
            if (i === "-") {
                i = r.next();
                const l = o();
                return (u)=>!!l && !l(u);
            }
            if (i === "(") {
                i = r.next();
                const l = a();
                return i === ")" && (i = r.next()), l;
            }
            if (Cn(i)) {
                const l = [];
                do l.push(i), i = r.next();
                while (Cn(i));
                return (u)=>e(l, u);
            }
            return null;
        }
        function s() {
            const l = [];
            let u = o();
            for(; u;)l.push(u), u = o();
            return (h)=>l.every((d)=>d(h));
        }
        function a() {
            const l = [];
            let u = s();
            for(; u && (l.push(u), i === "|" || i === ",");){
                do i = r.next();
                while (i === "|" || i === ",");
                u = s();
            }
            return (h)=>l.some((d)=>d(h));
        }
    }
    function Cn(t) {
        return !!t && !!t.match(/[\w\.:]+/);
    }
    function Ds(t) {
        let e = /([LR]:|[\w\.:][\w\.:\-]*|[\,\|\-\(\)])/g, n = e.exec(t);
        return {
            next: ()=>{
                if (!n) return null;
                const r = n[0];
                return n = e.exec(t), r;
            }
        };
    }
    function br(t) {
        typeof t.dispose == "function" && t.dispose();
    }
    var Me = class {
        constructor(t){
            this.scopeName = t;
        }
        toKey() {
            return this.scopeName;
        }
    }, Ns = class {
        constructor(t, e){
            this.scopeName = t, this.ruleName = e;
        }
        toKey() {
            return `${this.scopeName}#${this.ruleName}`;
        }
    }, Vs = class {
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
    }, Ms = class {
        constructor(t, e){
            this.repo = t, this.initialScopeName = e, this.seenFullScopeRequests.add(this.initialScopeName), this.Q = [
                new Me(this.initialScopeName)
            ];
        }
        seenFullScopeRequests = new Set;
        seenPartialScopeRequests = new Set;
        Q;
        processQueue() {
            const t = this.Q;
            this.Q = [];
            const e = new Vs;
            for (const n of t)$s(n, this.initialScopeName, this.repo, e);
            for (const n of e.references)if (n instanceof Me) {
                if (this.seenFullScopeRequests.has(n.scopeName)) continue;
                this.seenFullScopeRequests.add(n.scopeName), this.Q.push(n);
            } else {
                if (this.seenFullScopeRequests.has(n.scopeName) || this.seenPartialScopeRequests.has(n.toKey())) continue;
                this.seenPartialScopeRequests.add(n.toKey()), this.Q.push(n);
            }
        }
    };
    function $s(t, e, n, r) {
        const i = n.lookup(t.scopeName);
        if (!i) {
            if (t.scopeName === e) throw new Error(`No grammar provided for <${e}>`);
            return;
        }
        const o = n.lookup(e);
        t instanceof Me ? at({
            baseGrammar: o,
            selfGrammar: i
        }, r) : Kt(t.ruleName, {
            baseGrammar: o,
            selfGrammar: i,
            repository: i.repository
        }, r);
        const s = n.injections(t.scopeName);
        if (s) for (const a of s)r.add(new Me(a));
    }
    function Kt(t, e, n) {
        if (e.repository && e.repository[t]) {
            const r = e.repository[t];
            pt([
                r
            ], e, n);
        }
    }
    function at(t, e) {
        t.selfGrammar.patterns && Array.isArray(t.selfGrammar.patterns) && pt(t.selfGrammar.patterns, {
            ...t,
            repository: t.selfGrammar.repository
        }, e), t.selfGrammar.injections && pt(Object.values(t.selfGrammar.injections), {
            ...t,
            repository: t.selfGrammar.repository
        }, e);
    }
    function pt(t, e, n) {
        for (const r of t){
            if (n.visitedRule.has(r)) continue;
            n.visitedRule.add(r);
            const i = r.repository ? dr({}, e.repository, r.repository) : e.repository;
            Array.isArray(r.patterns) && pt(r.patterns, {
                ...e,
                repository: i
            }, n);
            const o = r.include;
            if (!o) continue;
            const s = Er(o);
            switch(s.kind){
                case 0:
                    at({
                        ...e,
                        selfGrammar: e.baseGrammar
                    }, n);
                    break;
                case 1:
                    at(e, n);
                    break;
                case 2:
                    Kt(s.ruleName, {
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
                        s.kind === 4 ? Kt(s.ruleName, l, n) : at(l, n);
                    } else s.kind === 4 ? n.add(new Ns(s.scopeName, s.ruleName)) : n.add(new Me(s.scopeName));
                    break;
            }
        }
    }
    var js = class {
        kind = 0;
    }, Bs = class {
        kind = 1;
    }, Gs = class {
        constructor(t){
            this.ruleName = t;
        }
        kind = 2;
    }, Us = class {
        constructor(t){
            this.scopeName = t;
        }
        kind = 3;
    }, Hs = class {
        constructor(t, e){
            this.scopeName = t, this.ruleName = e;
        }
        kind = 4;
    };
    function Er(t) {
        if (t === "$base") return new js;
        if (t === "$self") return new Bs;
        const e = t.indexOf("#");
        if (e === -1) return new Us(t);
        if (e === 0) return new Gs(t.substring(1));
        {
            const n = t.substring(0, e), r = t.substring(e + 1);
            return new Hs(n, r);
        }
    }
    var Fs = /\\(\d+)/, In = /\\(\d+)/g, Ws = -1, wr = -2;
    var We = class {
        $location;
        id;
        _nameIsCapturing;
        _name;
        _contentNameIsCapturing;
        _contentName;
        constructor(t, e, n, r){
            this.$location = t, this.id = e, this._name = n || null, this._nameIsCapturing = et.hasCaptures(this._name), this._contentName = r || null, this._contentNameIsCapturing = et.hasCaptures(this._contentName);
        }
        get debugName() {
            const t = this.$location ? `${hr(this.$location.filename)}:${this.$location.line}` : "unknown";
            return `${this.constructor.name}#${this.id} @ ${t}`;
        }
        getName(t, e) {
            return !this._nameIsCapturing || this._name === null || t === null || e === null ? this._name : et.replaceCaptures(this._name, t, e);
        }
        getContentName(t, e) {
            return !this._contentNameIsCapturing || this._contentName === null ? this._contentName : et.replaceCaptures(this._contentName, t, e);
        }
    }, zs = class extends We {
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
    }, qs = class extends We {
        _match;
        captures;
        _cachedCompiledPatterns;
        constructor(t, e, n, r, i){
            super(t, e, n, null), this._match = new $e(r, this.id), this.captures = i, this._cachedCompiledPatterns = null;
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
            return this._cachedCompiledPatterns || (this._cachedCompiledPatterns = new je, this.collectPatterns(t, this._cachedCompiledPatterns)), this._cachedCompiledPatterns;
        }
    }, On = class extends We {
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
            return this._cachedCompiledPatterns || (this._cachedCompiledPatterns = new je, this.collectPatterns(t, this._cachedCompiledPatterns)), this._cachedCompiledPatterns;
        }
    }, Jt = class extends We {
        _begin;
        beginCaptures;
        _end;
        endHasBackReferences;
        endCaptures;
        applyEndPatternLast;
        hasMissingPatterns;
        patterns;
        _cachedCompiledPatterns;
        constructor(t, e, n, r, i, o, s, a, l, u){
            super(t, e, n, r), this._begin = new $e(i, this.id), this.beginCaptures = o, this._end = new $e(s || "￿", -1), this.endHasBackReferences = this._end.hasBackReferences, this.endCaptures = a, this.applyEndPatternLast = l || !1, this.patterns = u.patterns, this.hasMissingPatterns = u.hasMissingPatterns, this._cachedCompiledPatterns = null;
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
                this._cachedCompiledPatterns = new je;
                for (const n of this.patterns)t.getRule(n).collectPatterns(t, this._cachedCompiledPatterns);
                this.applyEndPatternLast ? this._cachedCompiledPatterns.push(this._end.hasBackReferences ? this._end.clone() : this._end) : this._cachedCompiledPatterns.unshift(this._end.hasBackReferences ? this._end.clone() : this._end);
            }
            return this._end.hasBackReferences && (this.applyEndPatternLast ? this._cachedCompiledPatterns.setSource(this._cachedCompiledPatterns.length() - 1, e) : this._cachedCompiledPatterns.setSource(0, e)), this._cachedCompiledPatterns;
        }
    }, _t = class extends We {
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
            super(t, e, n, r), this._begin = new $e(i, this.id), this.beginCaptures = o, this.whileCaptures = a, this._while = new $e(s, wr), this.whileHasBackReferences = this._while.hasBackReferences, this.patterns = l.patterns, this.hasMissingPatterns = l.hasMissingPatterns, this._cachedCompiledPatterns = null, this._cachedCompiledWhilePatterns = null;
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
                this._cachedCompiledPatterns = new je;
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
            return this._cachedCompiledWhilePatterns || (this._cachedCompiledWhilePatterns = new je, this._cachedCompiledWhilePatterns.push(this._while.hasBackReferences ? this._while.clone() : this._while)), this._while.hasBackReferences && this._cachedCompiledWhilePatterns.setSource(0, e || "￿"), this._cachedCompiledWhilePatterns;
        }
    }, vr = class U {
        static createCaptureRule(e, n, r, i, o) {
            return e.registerRule((s)=>new zs(n, s, r, i, o));
        }
        static getCompiledRuleId(e, n, r) {
            return e.id || n.registerRule((i)=>{
                if (e.id = i, e.match) return new qs(e.$vscodeTextmateLocation, e.id, e.name, e.match, U._compileCaptures(e.captures, n, r));
                if (typeof e.begin > "u") {
                    e.repository && (r = dr({}, r, e.repository));
                    let o = e.patterns;
                    return typeof o > "u" && e.include && (o = [
                        {
                            include: e.include
                        }
                    ]), new On(e.$vscodeTextmateLocation, e.id, e.name, e.contentName, U._compilePatterns(o, n, r));
                }
                return e.while ? new _t(e.$vscodeTextmateLocation, e.id, e.name, e.contentName, e.begin, U._compileCaptures(e.beginCaptures || e.captures, n, r), e.while, U._compileCaptures(e.whileCaptures || e.captures, n, r), U._compilePatterns(e.patterns, n, r)) : new Jt(e.$vscodeTextmateLocation, e.id, e.name, e.contentName, e.begin, U._compileCaptures(e.beginCaptures || e.captures, n, r), e.end, U._compileCaptures(e.endCaptures || e.captures, n, r), e.applyEndPatternLast, U._compilePatterns(e.patterns, n, r));
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
                    e[s].patterns && (l = U.getCompiledRuleId(e[s], n, r)), i[a] = U.createCaptureRule(n, e[s].$vscodeTextmateLocation, e[s].name, e[s].contentName, l);
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
                    const u = Er(a.include);
                    switch(u.kind){
                        case 0:
                        case 1:
                            l = U.getCompiledRuleId(r[a.include], n, r);
                            break;
                        case 2:
                            let h = r[u.ruleName];
                            h && (l = U.getCompiledRuleId(h, n, r));
                            break;
                        case 3:
                        case 4:
                            const d = u.scopeName, _ = u.kind === 4 ? u.ruleName : null, f = n.getExternalGrammar(d, r);
                            if (f) if (_) {
                                let m = f.repository[_];
                                m && (l = U.getCompiledRuleId(m, n, f.repository));
                            } else l = U.getCompiledRuleId(f.repository.$self, n, f.repository);
                            break;
                    }
                } else l = U.getCompiledRuleId(a, n, r);
                if (l !== -1) {
                    const u = n.getRule(l);
                    let h = !1;
                    if ((u instanceof On || u instanceof Jt || u instanceof _t) && u.hasMissingPatterns && u.patterns.length === 0 && (h = !0), h) continue;
                    i.push(l);
                }
            }
            return {
                patterns: i,
                hasMissingPatterns: (e ? e.length : 0) !== i.length
            };
        }
    }, $e = class Sr {
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
                    const u = e.charAt(a + 1);
                    u === "z" ? (o.push(e.substring(i, a)), o.push("$(?!\\n)(?<!\\n)"), i = a + 2) : (u === "A" || u === "G") && (s = !0), a++;
                }
                this.hasAnchor = s, i === 0 ? this.source = e : (o.push(e.substring(i, r)), this.source = o.join(""));
            } else this.hasAnchor = !1, this.source = e;
            this.hasAnchor ? this._anchorCache = this._buildAnchorCache() : this._anchorCache = null, this.ruleId = n, typeof this.source == "string" ? this.hasBackReferences = Fs.test(this.source) : this.hasBackReferences = !1;
        }
        clone() {
            return new Sr(this.source, this.ruleId);
        }
        setSource(e) {
            this.source !== e && (this.source = e, this.hasAnchor && (this._anchorCache = this._buildAnchorCache()));
        }
        resolveBackReferences(e, n) {
            if (typeof this.source != "string") throw new Error("This method should only be called if the source is a string");
            let r = n.map((i)=>e.substring(i.start, i.end));
            return In.lastIndex = 0, this.source.replace(In, (i, o)=>pr(r[parseInt(o, 10)] || ""));
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
    }, je = class {
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
                this._cached = new xn(t, e, this._items.map((n)=>n.ruleId));
            }
            return this._cached;
        }
        compileAG(t, e, n) {
            return this._hasAnchors ? e ? n ? (this._anchorCache.A1_G1 || (this._anchorCache.A1_G1 = this._resolveAnchors(t, e, n)), this._anchorCache.A1_G1) : (this._anchorCache.A1_G0 || (this._anchorCache.A1_G0 = this._resolveAnchors(t, e, n)), this._anchorCache.A1_G0) : n ? (this._anchorCache.A0_G1 || (this._anchorCache.A0_G1 = this._resolveAnchors(t, e, n)), this._anchorCache.A0_G1) : (this._anchorCache.A0_G0 || (this._anchorCache.A0_G0 = this._resolveAnchors(t, e, n)), this._anchorCache.A0_G0) : this.compile(t);
        }
        _resolveAnchors(t, e, n) {
            let r = this._items.map((i)=>i.resolveAnchors(e, n));
            return new xn(t, r, this._items.map((i)=>i.ruleId));
        }
    }, xn = class {
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
    }, $t = class {
        constructor(t, e){
            this.languageId = t, this.tokenType = e;
        }
    }, Ks = class Xt {
        _defaultAttributes;
        _embeddedLanguagesMatcher;
        constructor(e, n){
            this._defaultAttributes = new $t(e, 8), this._embeddedLanguagesMatcher = new Js(Object.entries(n || {}));
        }
        getDefaultAttributes() {
            return this._defaultAttributes;
        }
        getBasicScopeAttributes(e) {
            return e === null ? Xt._NULL_SCOPE_METADATA : this._getBasicScopeAttributes.get(e);
        }
        static _NULL_SCOPE_METADATA = new $t(0, 0);
        _getBasicScopeAttributes = new _r((e)=>{
            const n = this._scopeToLanguage(e), r = this._toStandardTokenType(e);
            return new $t(n, r);
        });
        _scopeToLanguage(e) {
            return this._embeddedLanguagesMatcher.match(e) || 0;
        }
        _toStandardTokenType(e) {
            const n = e.match(Xt.STANDARD_TOKEN_TYPE_REGEXP);
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
    }, Js = class {
        values;
        scopesRegExp;
        constructor(t){
            if (t.length === 0) this.values = null, this.scopesRegExp = null;
            else {
                this.values = new Map(t);
                const e = t.map(([n, r])=>pr(n));
                e.sort(), e.reverse(), this.scopesRegExp = new RegExp(`^((${e.join(")|(")}))($|\\.)`, "");
            }
        }
        match(t) {
            if (!this.scopesRegExp) return;
            const e = t.match(this.scopesRegExp);
            if (e) return this.values.get(e[1]);
        }
    }, Dn = class {
        constructor(t, e){
            this.stack = t, this.stoppedEarly = e;
        }
    };
    function Ar(t, e, n, r, i, o, s, a) {
        const l = e.content.length;
        let u = !1, h = -1;
        if (s) {
            const f = Xs(t, e, n, r, i, o);
            i = f.stack, r = f.linePos, n = f.isFirstLine, h = f.anchorPosition;
        }
        const d = Date.now();
        for(; !u;){
            if (a !== 0 && Date.now() - d > a) return new Dn(i, !0);
            _();
        }
        return new Dn(i, !1);
        function _() {
            const f = Ys(t, e, n, r, i, h);
            if (!f) {
                o.produce(i, l), u = !0;
                return;
            }
            const m = f.captureIndices, w = f.matchedRuleId, g = m && m.length > 0 ? m[0].end > r : !1;
            if (w === Ws) {
                const E = i.getRule(t);
                o.produce(i, m[0].start), i = i.withContentNameScopesList(i.nameScopesList), Oe(t, e, n, i, o, E.endCaptures, m), o.produce(i, m[0].end);
                const p = i;
                if (i = i.parent, h = p.getAnchorPos(), !g && p.getEnterPos() === r) {
                    i = p, o.produce(i, l), u = !0;
                    return;
                }
            } else {
                const E = t.getRule(w);
                o.produce(i, m[0].start);
                const p = i, b = E.getName(e.content, m), v = i.contentNameScopesList.pushAttributed(b, t);
                if (i = i.push(w, r, h, m[0].end === l, null, v, v), E instanceof Jt) {
                    const T = E;
                    Oe(t, e, n, i, o, T.beginCaptures, m), o.produce(i, m[0].end), h = m[0].end;
                    const O = T.getContentName(e.content, m), V = v.pushAttributed(O, t);
                    if (i = i.withContentNameScopesList(V), T.endHasBackReferences && (i = i.withEndRule(T.getEndWithResolvedBackReferences(e.content, m))), !g && p.hasSameRuleAs(i)) {
                        i = i.pop(), o.produce(i, l), u = !0;
                        return;
                    }
                } else if (E instanceof _t) {
                    const T = E;
                    Oe(t, e, n, i, o, T.beginCaptures, m), o.produce(i, m[0].end), h = m[0].end;
                    const O = T.getContentName(e.content, m), V = v.pushAttributed(O, t);
                    if (i = i.withContentNameScopesList(V), T.whileHasBackReferences && (i = i.withEndRule(T.getWhileWithResolvedBackReferences(e.content, m))), !g && p.hasSameRuleAs(i)) {
                        i = i.pop(), o.produce(i, l), u = !0;
                        return;
                    }
                } else if (Oe(t, e, n, i, o, E.captures, m), o.produce(i, m[0].end), i = i.pop(), !g) {
                    i = i.safePop(), o.produce(i, l), u = !0;
                    return;
                }
            }
            m[0].end > r && (r = m[0].end, n = !1);
        }
    }
    function Xs(t, e, n, r, i, o) {
        let s = i.beginRuleCapturedEOL ? 0 : -1;
        const a = [];
        for(let l = i; l; l = l.pop()){
            const u = l.getRule(t);
            u instanceof _t && a.push({
                rule: u,
                stack: l
            });
        }
        for(let l = a.pop(); l; l = a.pop()){
            const { ruleScanner: u, findOptions: h } = ea(l.rule, t, l.stack.endRule, n, r === s), d = u.findNextMatchSync(e, r, h);
            if (d) {
                if (d.ruleId !== wr) {
                    i = l.stack.pop();
                    break;
                }
                d.captureIndices && d.captureIndices.length && (o.produce(l.stack, d.captureIndices[0].start), Oe(t, e, n, l.stack, o, l.rule.whileCaptures, d.captureIndices), o.produce(l.stack, d.captureIndices[0].end), s = d.captureIndices[0].end, d.captureIndices[0].end > r && (r = d.captureIndices[0].end, n = !1));
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
    function Ys(t, e, n, r, i, o) {
        const s = Qs(t, e, n, r, i, o), a = t.getInjections();
        if (a.length === 0) return s;
        const l = Zs(a, t, e, n, r, i, o);
        if (!l) return s;
        if (!s) return l;
        const u = s.captureIndices[0].start, h = l.captureIndices[0].start;
        return h < u || l.priorityMatch && h === u ? l : s;
    }
    function Qs(t, e, n, r, i, o) {
        const s = i.getRule(t), { ruleScanner: a, findOptions: l } = Tr(s, t, i.endRule, n, r === o), u = a.findNextMatchSync(e, r, l);
        return u ? {
            captureIndices: u.captureIndices,
            matchedRuleId: u.ruleId
        } : null;
    }
    function Zs(t, e, n, r, i, o, s) {
        let a = Number.MAX_VALUE, l = null, u, h = 0;
        const d = o.contentNameScopesList.getScopeNames();
        for(let _ = 0, f = t.length; _ < f; _++){
            const m = t[_];
            if (!m.matcher(d)) continue;
            const w = e.getRule(m.ruleId), { ruleScanner: g, findOptions: E } = Tr(w, e, null, r, i === s), p = g.findNextMatchSync(n, i, E);
            if (!p) continue;
            const b = p.captureIndices[0].start;
            if (!(b >= a) && (a = b, l = p.captureIndices, u = p.ruleId, h = m.priority, a === i)) break;
        }
        return l ? {
            priorityMatch: h === -1,
            captureIndices: l,
            matchedRuleId: u
        } : null;
    }
    function Tr(t, e, n, r, i) {
        return {
            ruleScanner: t.compileAG(e, n, r, i),
            findOptions: 0
        };
    }
    function ea(t, e, n, r, i) {
        return {
            ruleScanner: t.compileWhileAG(e, n, r, i),
            findOptions: 0
        };
    }
    function Oe(t, e, n, r, i, o, s) {
        if (o.length === 0) return;
        const a = e.content, l = Math.min(o.length, s.length), u = [], h = s[0].end;
        for(let d = 0; d < l; d++){
            const _ = o[d];
            if (_ === null) continue;
            const f = s[d];
            if (f.length === 0) continue;
            if (f.start > h) break;
            for(; u.length > 0 && u[u.length - 1].endPos <= f.start;)i.produceFromScopes(u[u.length - 1].scopes, u[u.length - 1].endPos), u.pop();
            if (u.length > 0 ? i.produceFromScopes(u[u.length - 1].scopes, f.start) : i.produce(r, f.start), _.retokenizeCapturedWithRuleId) {
                const w = _.getName(a, s), g = r.contentNameScopesList.pushAttributed(w, t), E = _.getContentName(a, s), p = g.pushAttributed(E, t), b = r.push(_.retokenizeCapturedWithRuleId, f.start, -1, !1, null, g, p), v = t.createOnigString(a.substring(0, f.end));
                Ar(t, v, n && f.start === 0, f.start, b, i, !1, 0), br(v);
                continue;
            }
            const m = _.getName(a, s);
            if (m !== null) {
                const g = (u.length > 0 ? u[u.length - 1].scopes : r.contentNameScopesList).pushAttributed(m, t);
                u.push(new ta(g, f.end));
            }
        }
        for(; u.length > 0;)i.produceFromScopes(u[u.length - 1].scopes, u[u.length - 1].endPos), u.pop();
    }
    var ta = class {
        scopes;
        endPos;
        constructor(t, e){
            this.scopes = t, this.endPos = e;
        }
    };
    function na(t, e, n, r, i, o, s, a) {
        return new ia(t, e, n, r, i, o, s, a);
    }
    function Nn(t, e, n, r, i) {
        const o = ft(e, gt), s = vr.getCompiledRuleId(n, r, i.repository);
        for (const a of o)t.push({
            debugSelector: e,
            matcher: a.matcher,
            ruleId: s,
            grammar: i,
            priority: a.priority
        });
    }
    function gt(t, e) {
        if (e.length < t.length) return !1;
        let n = 0;
        return t.every((r)=>{
            for(let i = n; i < e.length; i++)if (ra(e[i], r)) return n = i + 1, !0;
            return !1;
        });
    }
    function ra(t, e) {
        if (!t) return !1;
        if (t === e) return !0;
        const n = e.length;
        return t.length > n && t.substr(0, n) === e && t[n] === ".";
    }
    var ia = class {
        constructor(t, e, n, r, i, o, s, a){
            if (this._rootScopeName = t, this.balancedBracketSelectors = o, this._onigLib = a, this._basicScopeAttributesProvider = new Ks(n, r), this._rootId = -1, this._lastRuleId = 0, this._ruleId2desc = [
                null
            ], this._includedGrammars = {}, this._grammarRepository = s, this._grammar = Vn(e, null), this._injections = null, this._tokenTypeMatchers = [], i) for (const l of Object.keys(i)){
                const u = ft(l, gt);
                for (const h of u)this._tokenTypeMatchers.push({
                    matcher: h.matcher,
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
                if (i) for(let s in i)Nn(e, s, i[s], this, r);
                const o = this._grammarRepository.injections(n);
                o && o.forEach((s)=>{
                    const a = this.getExternalGrammar(s);
                    if (a) {
                        const l = a.injectionSelector;
                        l && Nn(e, l, a, this, a);
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
                if (n) return this._includedGrammars[t] = Vn(n, e && e.$base), this._includedGrammars[t];
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
            this._rootId === -1 && (this._rootId = vr.getCompiledRuleId(this._grammar.repository.$self, this, this._grammar.repository), this.getInjections());
            let i;
            if (!e || e === Yt.NULL) {
                i = !0;
                const u = this._basicScopeAttributesProvider.getDefaultAttributes(), h = this.themeProvider.getDefaults(), d = Re.set(0, u.languageId, u.tokenType, null, h.fontStyle, h.foregroundId, h.backgroundId), _ = this.getRule(this._rootId).getName(null, null);
                let f;
                _ ? f = De.createRootAndLookUpScopeName(_, d, this) : f = De.createRoot("unknown", d), e = new Yt(null, this._rootId, -1, -1, !1, null, f, f);
            } else i = !1, e.reset();
            t = t + `
`;
            const o = this.createOnigString(t), s = o.content.length, a = new sa(n, t, this._tokenTypeMatchers, this.balancedBracketSelectors), l = Ar(this, o, i, 0, e, a, !0, r);
            return br(o), {
                lineLength: s,
                lineTokens: a,
                ruleStack: l.stack,
                stoppedEarly: l.stoppedEarly
            };
        }
    };
    function Vn(t, e) {
        return t = Ss(t), t.repository = t.repository || {}, t.repository.$self = {
            $vscodeTextmateLocation: t.$vscodeTextmateLocation,
            patterns: t.patterns,
            name: t.scopeName
        }, t.repository.$base = e || t.repository.$self, t;
    }
    var De = class ne {
        constructor(e, n, r){
            this.parent = e, this.scopePath = n, this.tokenAttributes = r;
        }
        static fromExtension(e, n) {
            let r = e, i = e?.scopePath ?? null;
            for (const o of n)i = Mt.push(i, o.scopeNames), r = new ne(r, i, o.encodedTokenAttributes);
            return r;
        }
        static createRoot(e, n) {
            return new ne(null, new Mt(null, e), n);
        }
        static createRootAndLookUpScopeName(e, n, r) {
            const i = r.getMetadataForScope(e), o = new Mt(null, e), s = r.themeProvider.themeMatch(o), a = ne.mergeAttributes(n, i, s);
            return new ne(null, o, a);
        }
        get scopeName() {
            return this.scopePath.scopeName;
        }
        toString() {
            return this.getScopeNames().join(" ");
        }
        equals(e) {
            return ne.equals(this, e);
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
            return r !== null && (i = r.fontStyle, o = r.foregroundId, s = r.backgroundId), Re.set(e, n.languageId, n.tokenType, null, i, o, s);
        }
        pushAttributed(e, n) {
            if (e === null) return this;
            if (e.indexOf(" ") === -1) return ne._pushAttributed(this, e, n);
            const r = e.split(/ /g);
            let i = this;
            for (const o of r)i = ne._pushAttributed(i, o, n);
            return i;
        }
        static _pushAttributed(e, n, r) {
            const i = r.getMetadataForScope(n), o = e.scopePath.push(n), s = r.themeProvider.themeMatch(o), a = ne.mergeAttributes(e.tokenAttributes, i, s);
            return new ne(e, o, a);
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
    }, Yt = class pe {
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
            return e === n ? !0 : this._structuralEquals(e, n) ? De.equals(e.contentNameScopesList, n.contentNameScopesList) : !1;
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
            const r = De.fromExtension(e?.nameScopesList ?? null, n.nameScopesList);
            return new pe(e, n.ruleId, n.enterPos ?? -1, n.anchorPos ?? -1, n.beginRuleCapturedEOL, n.endRule, r, De.fromExtension(r, n.contentNameScopesList));
        }
    }, oa = class {
        balancedBracketScopes;
        unbalancedBracketScopes;
        allowAny = !1;
        constructor(t, e){
            this.balancedBracketScopes = t.flatMap((n)=>n === "*" ? (this.allowAny = !0, []) : ft(n, gt).map((r)=>r.matcher)), this.unbalancedBracketScopes = e.flatMap((n)=>ft(n, gt).map((r)=>r.matcher));
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
    }, sa = class {
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
                    for (const s of this._tokenTypeOverrides)s.matcher(o) && (r = Re.set(r, 0, s.type, null, -1, 0, 0));
                    this.balancedBracketSelectors && (i = this.balancedBracketSelectors.match(o));
                }
                if (i && (r = Re.set(r, 0, 8, i, -1, 0, 0)), this._binaryTokens.length > 0 && this._binaryTokens[this._binaryTokens.length - 1] === r) {
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
    }, aa = class {
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
                this._grammars.set(t, na(t, o, e, n, r, i, this, this._onigLib));
            }
            return this._grammars.get(t);
        }
    }, la = class {
        _options;
        _syncRegistry;
        _ensureGrammarCache;
        constructor(e){
            this._options = e, this._syncRegistry = new aa(mt.createFromRawTheme(e.theme, e.colorMap), e.onigLib), this._ensureGrammarCache = new Map;
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
            return this._loadGrammar(e, n, r.embeddedLanguages, r.tokenTypes, new oa(r.balancedBracketSelectors || [], r.unbalancedBracketSelectors || []));
        }
        loadGrammar(e) {
            return this._loadGrammar(e, 0, null, null, null);
        }
        _loadGrammar(e, n, r, i, o) {
            const s = new Ms(this._syncRegistry, e);
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
    }, Qt = Yt.NULL;
    const ca = [
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
    class ze {
        constructor(e, n, r){
            this.normal = n, this.property = e, r && (this.space = r);
        }
    }
    ze.prototype.normal = {};
    ze.prototype.property = {};
    ze.prototype.space = void 0;
    function Rr(t, e) {
        const n = {}, r = {};
        for (const i of t)Object.assign(n, i.property), Object.assign(r, i.normal);
        return new ze(n, r, e);
    }
    function Zt(t) {
        return t.toLowerCase();
    }
    class q {
        constructor(e, n){
            this.attribute = n, this.property = e;
        }
    }
    q.prototype.attribute = "";
    q.prototype.booleanish = !1;
    q.prototype.boolean = !1;
    q.prototype.commaOrSpaceSeparated = !1;
    q.prototype.commaSeparated = !1;
    q.prototype.defined = !1;
    q.prototype.mustUseProperty = !1;
    q.prototype.number = !1;
    q.prototype.overloadedBoolean = !1;
    q.prototype.property = "";
    q.prototype.spaceSeparated = !1;
    q.prototype.space = void 0;
    let ua = 0;
    const A = be(), x = be(), en = be(), y = be(), k = be(), Se = be(), K = be();
    function be() {
        return 2 ** ++ua;
    }
    const tn = Object.freeze(Object.defineProperty({
        __proto__: null,
        boolean: A,
        booleanish: x,
        commaOrSpaceSeparated: K,
        commaSeparated: Se,
        number: y,
        overloadedBoolean: en,
        spaceSeparated: k
    }, Symbol.toStringTag, {
        value: "Module"
    })), jt = Object.keys(tn);
    class ln extends q {
        constructor(e, n, r, i){
            let o = -1;
            if (super(e, n), Mn(this, "space", i), typeof r == "number") for(; ++o < jt.length;){
                const s = jt[o];
                Mn(this, jt[o], (r & tn[s]) === tn[s]);
            }
        }
    }
    ln.prototype.defined = !0;
    function Mn(t, e, n) {
        n && (t[e] = n);
    }
    function Le(t) {
        const e = {}, n = {};
        for (const [r, i] of Object.entries(t.properties)){
            const o = new ln(r, t.transform(t.attributes || {}, r), i, t.space);
            t.mustUseProperty && t.mustUseProperty.includes(r) && (o.mustUseProperty = !0), e[r] = o, n[Zt(r)] = r, n[Zt(o.attribute)] = r;
        }
        return new ze(e, n, t.space);
    }
    const Lr = Le({
        properties: {
            ariaActiveDescendant: null,
            ariaAtomic: x,
            ariaAutoComplete: null,
            ariaBusy: x,
            ariaChecked: x,
            ariaColCount: y,
            ariaColIndex: y,
            ariaColSpan: y,
            ariaControls: k,
            ariaCurrent: null,
            ariaDescribedBy: k,
            ariaDetails: null,
            ariaDisabled: x,
            ariaDropEffect: k,
            ariaErrorMessage: null,
            ariaExpanded: x,
            ariaFlowTo: k,
            ariaGrabbed: x,
            ariaHasPopup: null,
            ariaHidden: x,
            ariaInvalid: null,
            ariaKeyShortcuts: null,
            ariaLabel: null,
            ariaLabelledBy: k,
            ariaLevel: y,
            ariaLive: null,
            ariaModal: x,
            ariaMultiLine: x,
            ariaMultiSelectable: x,
            ariaOrientation: null,
            ariaOwns: k,
            ariaPlaceholder: null,
            ariaPosInSet: y,
            ariaPressed: x,
            ariaReadOnly: x,
            ariaRelevant: null,
            ariaRequired: x,
            ariaRoleDescription: k,
            ariaRowCount: y,
            ariaRowIndex: y,
            ariaRowSpan: y,
            ariaSelected: x,
            ariaSetSize: y,
            ariaSort: null,
            ariaValueMax: y,
            ariaValueMin: y,
            ariaValueNow: y,
            ariaValueText: null,
            role: null
        },
        transform (t, e) {
            return e === "role" ? e : "aria-" + e.slice(4).toLowerCase();
        }
    });
    function Pr(t, e) {
        return e in t ? t[e] : e;
    }
    function kr(t, e) {
        return Pr(t, e.toLowerCase());
    }
    const da = Le({
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
            accept: Se,
            acceptCharset: k,
            accessKey: k,
            action: null,
            allow: null,
            allowFullScreen: A,
            allowPaymentRequest: A,
            allowUserMedia: A,
            alt: null,
            as: null,
            async: A,
            autoCapitalize: null,
            autoComplete: k,
            autoFocus: A,
            autoPlay: A,
            blocking: k,
            capture: null,
            charSet: null,
            checked: A,
            cite: null,
            className: k,
            cols: y,
            colSpan: null,
            content: null,
            contentEditable: x,
            controls: A,
            controlsList: k,
            coords: y | Se,
            crossOrigin: null,
            data: null,
            dateTime: null,
            decoding: null,
            default: A,
            defer: A,
            dir: null,
            dirName: null,
            disabled: A,
            download: en,
            draggable: x,
            encType: null,
            enterKeyHint: null,
            fetchPriority: null,
            form: null,
            formAction: null,
            formEncType: null,
            formMethod: null,
            formNoValidate: A,
            formTarget: null,
            headers: k,
            height: y,
            hidden: en,
            high: y,
            href: null,
            hrefLang: null,
            htmlFor: k,
            httpEquiv: k,
            id: null,
            imageSizes: null,
            imageSrcSet: null,
            inert: A,
            inputMode: null,
            integrity: null,
            is: null,
            isMap: A,
            itemId: null,
            itemProp: k,
            itemRef: k,
            itemScope: A,
            itemType: k,
            kind: null,
            label: null,
            lang: null,
            language: null,
            list: null,
            loading: null,
            loop: A,
            low: y,
            manifest: null,
            max: null,
            maxLength: y,
            media: null,
            method: null,
            min: null,
            minLength: y,
            multiple: A,
            muted: A,
            name: null,
            nonce: null,
            noModule: A,
            noValidate: A,
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
            open: A,
            optimum: y,
            pattern: null,
            ping: k,
            placeholder: null,
            playsInline: A,
            popover: null,
            popoverTarget: null,
            popoverTargetAction: null,
            poster: null,
            preload: null,
            readOnly: A,
            referrerPolicy: null,
            rel: k,
            required: A,
            reversed: A,
            rows: y,
            rowSpan: y,
            sandbox: k,
            scope: null,
            scoped: A,
            seamless: A,
            selected: A,
            shadowRootClonable: A,
            shadowRootDelegatesFocus: A,
            shadowRootMode: null,
            shape: null,
            size: y,
            sizes: null,
            slot: null,
            span: y,
            spellCheck: x,
            src: null,
            srcDoc: null,
            srcLang: null,
            srcSet: null,
            start: y,
            step: null,
            style: null,
            tabIndex: y,
            target: null,
            title: null,
            translate: null,
            type: null,
            typeMustMatch: A,
            useMap: null,
            value: x,
            width: y,
            wrap: null,
            writingSuggestions: null,
            align: null,
            aLink: null,
            archive: k,
            axis: null,
            background: null,
            bgColor: null,
            border: y,
            borderColor: null,
            bottomMargin: y,
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
            compact: A,
            declare: A,
            event: null,
            face: null,
            frame: null,
            frameBorder: null,
            hSpace: y,
            leftMargin: y,
            link: null,
            longDesc: null,
            lowSrc: null,
            marginHeight: y,
            marginWidth: y,
            noResize: A,
            noHref: A,
            noShade: A,
            noWrap: A,
            object: null,
            profile: null,
            prompt: null,
            rev: null,
            rightMargin: y,
            rules: null,
            scheme: null,
            scrolling: x,
            standby: null,
            summary: null,
            text: null,
            topMargin: y,
            valueType: null,
            version: null,
            vAlign: null,
            vLink: null,
            vSpace: y,
            allowTransparency: null,
            autoCorrect: null,
            autoSave: null,
            disablePictureInPicture: A,
            disableRemotePlayback: A,
            prefix: null,
            property: null,
            results: y,
            security: null,
            unselectable: null
        },
        space: "html",
        transform: kr
    }), ha = Le({
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
            accentHeight: y,
            accumulate: null,
            additive: null,
            alignmentBaseline: null,
            alphabetic: y,
            amplitude: y,
            arabicForm: null,
            ascent: y,
            attributeName: null,
            attributeType: null,
            azimuth: y,
            bandwidth: null,
            baselineShift: null,
            baseFrequency: null,
            baseProfile: null,
            bbox: null,
            begin: null,
            bias: y,
            by: null,
            calcMode: null,
            capHeight: y,
            className: k,
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
            descent: y,
            diffuseConstant: y,
            direction: null,
            display: null,
            dur: null,
            divisor: y,
            dominantBaseline: null,
            download: A,
            dx: null,
            dy: null,
            edgeMode: null,
            editable: null,
            elevation: y,
            enableBackground: null,
            end: null,
            event: null,
            exponent: y,
            externalResourcesRequired: null,
            fill: null,
            fillOpacity: y,
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
            g1: Se,
            g2: Se,
            glyphName: Se,
            glyphOrientationHorizontal: null,
            glyphOrientationVertical: null,
            glyphRef: null,
            gradientTransform: null,
            gradientUnits: null,
            handler: null,
            hanging: y,
            hatchContentUnits: null,
            hatchUnits: null,
            height: null,
            href: null,
            hrefLang: null,
            horizAdvX: y,
            horizOriginX: y,
            horizOriginY: y,
            id: null,
            ideographic: y,
            imageRendering: null,
            initialVisibility: null,
            in: null,
            in2: null,
            intercept: y,
            k: y,
            k1: y,
            k2: y,
            k3: y,
            k4: y,
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
            limitingConeAngle: y,
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
            mediaSize: y,
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
            overlinePosition: y,
            overlineThickness: y,
            paintOrder: null,
            panose1: null,
            path: null,
            pathLength: y,
            patternContentUnits: null,
            patternTransform: null,
            patternUnits: null,
            phase: null,
            ping: k,
            pitch: null,
            playbackOrder: null,
            pointerEvents: null,
            points: null,
            pointsAtX: y,
            pointsAtY: y,
            pointsAtZ: y,
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
            specularConstant: y,
            specularExponent: y,
            spreadMethod: null,
            spacing: null,
            startOffset: null,
            stdDeviation: null,
            stemh: null,
            stemv: null,
            stitchTiles: null,
            stopColor: null,
            stopOpacity: null,
            strikethroughPosition: y,
            strikethroughThickness: y,
            string: null,
            stroke: null,
            strokeDashArray: K,
            strokeDashOffset: null,
            strokeLineCap: null,
            strokeLineJoin: null,
            strokeMiterLimit: y,
            strokeOpacity: y,
            strokeWidth: null,
            style: null,
            surfaceScale: y,
            syncBehavior: null,
            syncBehaviorDefault: null,
            syncMaster: null,
            syncTolerance: null,
            syncToleranceDefault: null,
            systemLanguage: K,
            tabIndex: y,
            tableValues: null,
            target: null,
            targetX: y,
            targetY: y,
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
            underlinePosition: y,
            underlineThickness: y,
            unicode: null,
            unicodeBidi: null,
            unicodeRange: null,
            unitsPerEm: y,
            values: null,
            vAlphabetic: y,
            vMathematical: y,
            vectorEffect: null,
            vHanging: y,
            vIdeographic: y,
            version: null,
            vertAdvY: y,
            vertOriginX: y,
            vertOriginY: y,
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
            xHeight: y,
            y: null,
            y1: null,
            y2: null,
            yChannelSelector: null,
            z: null,
            zoomAndPan: null
        },
        space: "svg",
        transform: Pr
    }), Cr = Le({
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
    }), Ir = Le({
        attributes: {
            xmlnsxlink: "xmlns:xlink"
        },
        properties: {
            xmlnsXLink: null,
            xmlns: null
        },
        space: "xmlns",
        transform: kr
    }), Or = Le({
        properties: {
            xmlBase: null,
            xmlLang: null,
            xmlSpace: null
        },
        space: "xml",
        transform (t, e) {
            return "xml:" + e.slice(3).toLowerCase();
        }
    }), ma = /[A-Z]/g, $n = /-[a-z]/g, fa = /^data[-\w.:]+$/i;
    function pa(t, e) {
        const n = Zt(e);
        let r = e, i = q;
        if (n in t.normal) return t.property[t.normal[n]];
        if (n.length > 4 && n.slice(0, 4) === "data" && fa.test(e)) {
            if (e.charAt(4) === "-") {
                const o = e.slice(5).replace($n, ga);
                r = "data" + o.charAt(0).toUpperCase() + o.slice(1);
            } else {
                const o = e.slice(4);
                if (!$n.test(o)) {
                    let s = o.replace(ma, _a);
                    s.charAt(0) !== "-" && (s = "-" + s), e = "data" + s;
                }
            }
            i = ln;
        }
        return new i(r, e);
    }
    function _a(t) {
        return "-" + t.toLowerCase();
    }
    function ga(t) {
        return t.charAt(1).toUpperCase();
    }
    const ya = Rr([
        Lr,
        da,
        Cr,
        Ir,
        Or
    ], "html"), xr = Rr([
        Lr,
        ha,
        Cr,
        Ir,
        Or
    ], "svg"), jn = {}.hasOwnProperty;
    function ba(t, e) {
        const n = e || {};
        function r(i, ...o) {
            let s = r.invalid;
            const a = r.handlers;
            if (i && jn.call(i, t)) {
                const l = String(i[t]);
                s = jn.call(a, l) ? a[l] : r.unknown;
            }
            if (s) return s.call(this, i, ...o);
        }
        return r.handlers = n.handlers || {}, r.invalid = n.invalid, r.unknown = n.unknown, r;
    }
    const Ea = /["&'<>`]/g, wa = /[\uD800-\uDBFF][\uDC00-\uDFFF]/g, va = /[\x01-\t\v\f\x0E-\x1F\x7F\x81\x8D\x8F\x90\x9D\xA0-\uFFFF]/g, Sa = /[|\\{}()[\]^$+*?.]/g, Bn = new WeakMap;
    function Aa(t, e) {
        if (t = t.replace(e.subset ? Ta(e.subset) : Ea, r), e.subset || e.escapeOnly) return t;
        return t.replace(wa, n).replace(va, r);
        function n(i, o, s) {
            return e.format((i.charCodeAt(0) - 55296) * 1024 + i.charCodeAt(1) - 56320 + 65536, s.charCodeAt(o + 2), e);
        }
        function r(i, o, s) {
            return e.format(i.charCodeAt(0), s.charCodeAt(o + 1), e);
        }
    }
    function Ta(t) {
        let e = Bn.get(t);
        return e || (e = Ra(t), Bn.set(t, e)), e;
    }
    function Ra(t) {
        const e = [];
        let n = -1;
        for(; ++n < t.length;)e.push(t[n].replace(Sa, "\\$&"));
        return new RegExp("(?:" + e.join("|") + ")", "g");
    }
    const La = /[\dA-Fa-f]/;
    function Pa(t, e, n) {
        const r = "&#x" + t.toString(16).toUpperCase();
        return n && e && !La.test(String.fromCharCode(e)) ? r : r + ";";
    }
    const ka = /\d/;
    function Ca(t, e, n) {
        const r = "&#" + String(t);
        return n && e && !ka.test(String.fromCharCode(e)) ? r : r + ";";
    }
    const Ia = [
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
    ], Bt = {
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
    }, Oa = [
        "cent",
        "copy",
        "divide",
        "gt",
        "lt",
        "not",
        "para",
        "times"
    ], Dr = {}.hasOwnProperty, nn = {};
    let tt;
    for(tt in Bt)Dr.call(Bt, tt) && (nn[Bt[tt]] = tt);
    const xa = /[^\dA-Za-z]/;
    function Da(t, e, n, r) {
        const i = String.fromCharCode(t);
        if (Dr.call(nn, i)) {
            const o = nn[i], s = "&" + o;
            return n && Ia.includes(o) && !Oa.includes(o) && (!r || e && e !== 61 && xa.test(String.fromCharCode(e))) ? s : s + ";";
        }
        return "";
    }
    function Na(t, e, n) {
        let r = Pa(t, e, n.omitOptionalSemicolons), i;
        if ((n.useNamedReferences || n.useShortestReferences) && (i = Da(t, e, n.omitOptionalSemicolons, n.attribute)), (n.useShortestReferences || !i) && n.useShortestReferences) {
            const o = Ca(t, e, n.omitOptionalSemicolons);
            o.length < r.length && (r = o);
        }
        return i && (!n.useShortestReferences || i.length < r.length) ? i : r;
    }
    function Ae(t, e) {
        return Aa(t, Object.assign({
            format: Na
        }, e));
    }
    const Va = /^>|^->|<!--|-->|--!>|<!-$/g, Ma = [
        ">"
    ], $a = [
        "<",
        ">"
    ];
    function ja(t, e, n, r) {
        return r.settings.bogusComments ? "<?" + Ae(t.value, Object.assign({}, r.settings.characterReferences, {
            subset: Ma
        })) + ">" : "<!--" + t.value.replace(Va, i) + "-->";
        function i(o) {
            return Ae(o, Object.assign({}, r.settings.characterReferences, {
                subset: $a
            }));
        }
    }
    function Ba(t, e, n, r) {
        return "<!" + (r.settings.upperDoctype ? "DOCTYPE" : "doctype") + (r.settings.tightDoctype ? "" : " ") + "html>";
    }
    function Gn(t, e) {
        const n = String(t);
        if (typeof e != "string") throw new TypeError("Expected character");
        let r = 0, i = n.indexOf(e);
        for(; i !== -1;)r++, i = n.indexOf(e, i + e.length);
        return r;
    }
    function Ga(t, e) {
        const n = e || {};
        return (t[t.length - 1] === "" ? [
            ...t,
            ""
        ] : t).join((n.padRight ? " " : "") + "," + (n.padLeft === !1 ? "" : " ")).trim();
    }
    function Ua(t) {
        return t.join(" ").trim();
    }
    const Ha = /[ \t\n\f\r]/g;
    function cn(t) {
        return typeof t == "object" ? t.type === "text" ? Un(t.value) : !1 : Un(t);
    }
    function Un(t) {
        return t.replace(Ha, "") === "";
    }
    const j = Vr(1), Nr = Vr(-1), Fa = [];
    function Vr(t) {
        return e;
        function e(n, r, i) {
            const o = n ? n.children : Fa;
            let s = (r || 0) + t, a = o[s];
            if (!i) for(; a && cn(a);)s += t, a = o[s];
            return a;
        }
    }
    const Wa = {}.hasOwnProperty;
    function Mr(t) {
        return e;
        function e(n, r, i) {
            return Wa.call(t, n.tagName) && t[n.tagName](n, r, i);
        }
    }
    const un = Mr({
        body: qa,
        caption: Gt,
        colgroup: Gt,
        dd: Ya,
        dt: Xa,
        head: Gt,
        html: za,
        li: Ja,
        optgroup: Qa,
        option: Za,
        p: Ka,
        rp: Hn,
        rt: Hn,
        tbody: tl,
        td: Fn,
        tfoot: nl,
        th: Fn,
        thead: el,
        tr: rl
    });
    function Gt(t, e, n) {
        const r = j(n, e, !0);
        return !r || r.type !== "comment" && !(r.type === "text" && cn(r.value.charAt(0)));
    }
    function za(t, e, n) {
        const r = j(n, e);
        return !r || r.type !== "comment";
    }
    function qa(t, e, n) {
        const r = j(n, e);
        return !r || r.type !== "comment";
    }
    function Ka(t, e, n) {
        const r = j(n, e);
        return r ? r.type === "element" && (r.tagName === "address" || r.tagName === "article" || r.tagName === "aside" || r.tagName === "blockquote" || r.tagName === "details" || r.tagName === "div" || r.tagName === "dl" || r.tagName === "fieldset" || r.tagName === "figcaption" || r.tagName === "figure" || r.tagName === "footer" || r.tagName === "form" || r.tagName === "h1" || r.tagName === "h2" || r.tagName === "h3" || r.tagName === "h4" || r.tagName === "h5" || r.tagName === "h6" || r.tagName === "header" || r.tagName === "hgroup" || r.tagName === "hr" || r.tagName === "main" || r.tagName === "menu" || r.tagName === "nav" || r.tagName === "ol" || r.tagName === "p" || r.tagName === "pre" || r.tagName === "section" || r.tagName === "table" || r.tagName === "ul") : !n || !(n.type === "element" && (n.tagName === "a" || n.tagName === "audio" || n.tagName === "del" || n.tagName === "ins" || n.tagName === "map" || n.tagName === "noscript" || n.tagName === "video"));
    }
    function Ja(t, e, n) {
        const r = j(n, e);
        return !r || r.type === "element" && r.tagName === "li";
    }
    function Xa(t, e, n) {
        const r = j(n, e);
        return !!(r && r.type === "element" && (r.tagName === "dt" || r.tagName === "dd"));
    }
    function Ya(t, e, n) {
        const r = j(n, e);
        return !r || r.type === "element" && (r.tagName === "dt" || r.tagName === "dd");
    }
    function Hn(t, e, n) {
        const r = j(n, e);
        return !r || r.type === "element" && (r.tagName === "rp" || r.tagName === "rt");
    }
    function Qa(t, e, n) {
        const r = j(n, e);
        return !r || r.type === "element" && r.tagName === "optgroup";
    }
    function Za(t, e, n) {
        const r = j(n, e);
        return !r || r.type === "element" && (r.tagName === "option" || r.tagName === "optgroup");
    }
    function el(t, e, n) {
        const r = j(n, e);
        return !!(r && r.type === "element" && (r.tagName === "tbody" || r.tagName === "tfoot"));
    }
    function tl(t, e, n) {
        const r = j(n, e);
        return !r || r.type === "element" && (r.tagName === "tbody" || r.tagName === "tfoot");
    }
    function nl(t, e, n) {
        return !j(n, e);
    }
    function rl(t, e, n) {
        const r = j(n, e);
        return !r || r.type === "element" && r.tagName === "tr";
    }
    function Fn(t, e, n) {
        const r = j(n, e);
        return !r || r.type === "element" && (r.tagName === "td" || r.tagName === "th");
    }
    const il = Mr({
        body: al,
        colgroup: ll,
        head: sl,
        html: ol,
        tbody: cl
    });
    function ol(t) {
        const e = j(t, -1);
        return !e || e.type !== "comment";
    }
    function sl(t) {
        const e = new Set;
        for (const r of t.children)if (r.type === "element" && (r.tagName === "base" || r.tagName === "title")) {
            if (e.has(r.tagName)) return !1;
            e.add(r.tagName);
        }
        const n = t.children[0];
        return !n || n.type === "element";
    }
    function al(t) {
        const e = j(t, -1, !0);
        return !e || e.type !== "comment" && !(e.type === "text" && cn(e.value.charAt(0))) && !(e.type === "element" && (e.tagName === "meta" || e.tagName === "link" || e.tagName === "script" || e.tagName === "style" || e.tagName === "template"));
    }
    function ll(t, e, n) {
        const r = Nr(n, e), i = j(t, -1, !0);
        return n && r && r.type === "element" && r.tagName === "colgroup" && un(r, n.children.indexOf(r), n) ? !1 : !!(i && i.type === "element" && i.tagName === "col");
    }
    function cl(t, e, n) {
        const r = Nr(n, e), i = j(t, -1);
        return n && r && r.type === "element" && (r.tagName === "thead" || r.tagName === "tbody") && un(r, n.children.indexOf(r), n) ? !1 : !!(i && i.type === "element" && i.tagName === "tr");
    }
    const nt = {
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
    function ul(t, e, n, r) {
        const i = r.schema, o = i.space === "svg" ? !1 : r.settings.omitOptionalTags;
        let s = i.space === "svg" ? r.settings.closeEmptyElements : r.settings.voids.includes(t.tagName.toLowerCase());
        const a = [];
        let l;
        i.space === "html" && t.tagName === "svg" && (r.schema = xr);
        const u = dl(r, t.properties), h = r.all(i.space === "html" && t.tagName === "template" ? t.content : t);
        return r.schema = i, h && (s = !1), (u || !o || !il(t, e, n)) && (a.push("<", t.tagName, u ? " " + u : ""), s && (i.space === "svg" || r.settings.closeSelfClosing) && (l = u.charAt(u.length - 1), (!r.settings.tightSelfClosing || l === "/" || l && l !== '"' && l !== "'") && a.push(" "), a.push("/")), a.push(">")), a.push(h), !s && (!o || !un(t, e, n)) && a.push("</" + t.tagName + ">"), a.join("");
    }
    function dl(t, e) {
        const n = [];
        let r = -1, i;
        if (e) {
            for(i in e)if (e[i] !== null && e[i] !== void 0) {
                const o = hl(t, i, e[i]);
                o && n.push(o);
            }
        }
        for(; ++r < n.length;){
            const o = t.settings.tightAttributes ? n[r].charAt(n[r].length - 1) : void 0;
            r !== n.length - 1 && o !== '"' && o !== "'" && (n[r] += " ");
        }
        return n.join("");
    }
    function hl(t, e, n) {
        const r = pa(t.schema, e), i = t.settings.allowParseErrors && t.schema.space === "html" ? 0 : 1, o = t.settings.allowDangerousCharacters ? 0 : 1;
        let s = t.quote, a;
        if (r.overloadedBoolean && (n === r.attribute || n === "") ? n = !0 : (r.boolean || r.overloadedBoolean) && (typeof n != "string" || n === r.attribute || n === "") && (n = !!n), n == null || n === !1 || typeof n == "number" && Number.isNaN(n)) return "";
        const l = Ae(r.attribute, Object.assign({}, t.settings.characterReferences, {
            subset: nt.name[i][o]
        }));
        return n === !0 || (n = Array.isArray(n) ? (r.commaSeparated ? Ga : Ua)(n, {
            padLeft: !t.settings.tightCommaSeparatedLists
        }) : String(n), t.settings.collapseEmptyAttributes && !n) ? l : (t.settings.preferUnquoted && (a = Ae(n, Object.assign({}, t.settings.characterReferences, {
            attribute: !0,
            subset: nt.unquoted[i][o]
        }))), a !== n && (t.settings.quoteSmart && Gn(n, s) > Gn(n, t.alternative) && (s = t.alternative), a = s + Ae(n, Object.assign({}, t.settings.characterReferences, {
            subset: (s === "'" ? nt.single : nt.double)[i][o],
            attribute: !0
        })) + s), l + (a && "=" + a));
    }
    const ml = [
        "<",
        "&"
    ];
    function $r(t, e, n, r) {
        return n && n.type === "element" && (n.tagName === "script" || n.tagName === "style") ? t.value : Ae(t.value, Object.assign({}, r.settings.characterReferences, {
            subset: ml
        }));
    }
    function fl(t, e, n, r) {
        return r.settings.allowDangerousHtml ? t.value : $r(t, e, n, r);
    }
    function pl(t, e, n, r) {
        return r.all(t);
    }
    const _l = ba("type", {
        invalid: gl,
        unknown: yl,
        handlers: {
            comment: ja,
            doctype: Ba,
            element: ul,
            raw: fl,
            root: pl,
            text: $r
        }
    });
    function gl(t) {
        throw new Error("Expected node, not `" + t + "`");
    }
    function yl(t) {
        const e = t;
        throw new Error("Cannot compile unknown node `" + e.type + "`");
    }
    const bl = {}, El = {}, wl = [];
    function vl(t, e) {
        const n = bl, r = n.quote || '"', i = r === '"' ? "'" : '"';
        if (r !== '"' && r !== "'") throw new Error("Invalid quote `" + r + "`, expected `'` or `\"`");
        return {
            one: Sl,
            all: Al,
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
                voids: n.voids || ca,
                characterReferences: n.characterReferences || El,
                closeSelfClosing: n.closeSelfClosing || !1,
                closeEmptyElements: n.closeEmptyElements || !1
            },
            schema: n.space === "svg" ? xr : ya,
            quote: r,
            alternative: i
        }.one(Array.isArray(t) ? {
            type: "root",
            children: t
        } : t, void 0, void 0);
    }
    function Sl(t, e, n) {
        return _l(t, e, n, this);
    }
    function Al(t) {
        const e = [], n = t && t.children || wl;
        let r = -1;
        for(; ++r < n.length;)e[r] = this.one(n[r], r, t);
        return e.join("");
    }
    function Tl(t) {
        return Array.isArray(t) ? t : [
            t
        ];
    }
    function Lt(t, e = !1) {
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
    function dn(t) {
        return !t || [
            "plaintext",
            "txt",
            "text",
            "plain"
        ].includes(t);
    }
    function jr(t) {
        return t === "ansi" || dn(t);
    }
    function hn(t) {
        return t === "none";
    }
    function Br(t) {
        return hn(t);
    }
    function Gr(t, e) {
        if (!e) return t;
        t.properties ||= {}, t.properties.class ||= [], typeof t.properties.class == "string" && (t.properties.class = t.properties.class.split(/\s+/g)), Array.isArray(t.properties.class) || (t.properties.class = []);
        const n = Array.isArray(e) ? e : e.split(/\s+/g);
        for (const r of n)r && !t.properties.class.includes(r) && t.properties.class.push(r);
        return t;
    }
    function Rl(t, e) {
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
    function Ll(t, e) {
        const n = Array.from(e instanceof Set ? e : new Set(e)).sort((r, i)=>r - i);
        return n.length ? t.map((r)=>r.flatMap((i)=>{
                const o = n.filter((s)=>i.offset < s && s < i.offset + i.content.length).map((s)=>s - i.offset).sort((s, a)=>s - a);
                return o.length ? Rl(i, o) : i;
            })) : t;
    }
    async function Ur(t) {
        return Promise.resolve(typeof t == "function" ? t() : t).then((e)=>e.default || e);
    }
    function yt(t, e) {
        const n = typeof t == "string" ? {} : {
            ...t.colorReplacements
        }, r = typeof t == "string" ? t : t.name;
        for (const [i, o] of Object.entries(e?.colorReplacements || {}))typeof o == "string" ? n[i] = o : i === r && Object.assign(n, o);
        return n;
    }
    function _e(t, e) {
        return t && (e?.[t?.toLowerCase()] || t);
    }
    function Hr(t) {
        const e = {};
        return t.color && (e.color = t.color), t.bgColor && (e["background-color"] = t.bgColor), t.fontStyle && (t.fontStyle & ae.Italic && (e["font-style"] = "italic"), t.fontStyle & ae.Bold && (e["font-weight"] = "bold"), t.fontStyle & ae.Underline && (e["text-decoration"] = "underline")), e;
    }
    function Pl(t) {
        return typeof t == "string" ? t : Object.entries(t).map(([e, n])=>`${e}:${n}`).join(";");
    }
    function kl(t) {
        const e = Lt(t, !0).map(([i])=>i);
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
    const Fr = new WeakMap;
    function Pt(t, e) {
        Fr.set(t, e);
    }
    function Be(t) {
        return Fr.get(t);
    }
    class Pe {
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
            return new Pe(Object.fromEntries(Tl(n).map((r)=>[
                    r,
                    Qt
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
            return Wn(this._stacks[this.theme]);
        }
        getScopes(e = this.theme) {
            return Wn(this._stacks[e]);
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
    function Wn(t) {
        const e = [], n = new Set;
        function r(i) {
            if (n.has(i)) return;
            n.add(i);
            const o = i?.nameScopesList?.scopeName;
            o && e.push(o), i.parent && r(i.parent);
        }
        return r(t), e;
    }
    function Cl(t, e) {
        if (!(t instanceof Pe)) throw new W("Invalid grammar state");
        return t.getInternalStack(e);
    }
    function Il() {
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
                const i = kl(n.source), o = (n.options.decorations || []).map((s)=>({
                        ...s,
                        start: r(s.start),
                        end: r(s.end)
                    }));
                Ol(o), t.set(n.meta, {
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
                return Ll(n, i);
            },
            code (n) {
                if (!this.options.decorations?.length) return;
                const r = e(this), i = Array.from(n.children).filter((h)=>h.type === "element" && h.tagName === "span");
                if (i.length !== r.converter.lines.length) throw new W(`Number of lines in code element (${i.length}) does not match the number of lines in the source (${r.converter.lines.length}). Failed to apply decorations.`);
                function o(h, d, _, f) {
                    const m = i[h];
                    let w = "", g = -1, E = -1;
                    if (d === 0 && (g = 0), _ === 0 && (E = 0), _ === Number.POSITIVE_INFINITY && (E = m.children.length), g === -1 || E === -1) for(let b = 0; b < m.children.length; b++)w += Wr(m.children[b]), g === -1 && w.length === d && (g = b + 1), E === -1 && w.length === _ && (E = b + 1);
                    if (g === -1) throw new W(`Failed to find start index for decoration ${JSON.stringify(f.start)}`);
                    if (E === -1) throw new W(`Failed to find end index for decoration ${JSON.stringify(f.end)}`);
                    const p = m.children.slice(g, E);
                    if (!f.alwaysWrap && p.length === m.children.length) a(m, f, "line");
                    else if (!f.alwaysWrap && p.length === 1 && p[0].type === "element") a(p[0], f, "token");
                    else {
                        const b = {
                            type: "element",
                            tagName: "span",
                            properties: {},
                            children: p
                        };
                        a(b, f, "wrapper"), m.children.splice(g, p.length, b);
                    }
                }
                function s(h, d) {
                    i[h] = a(i[h], d, "line");
                }
                function a(h, d, _) {
                    const f = d.properties || {}, m = d.transform || ((w)=>w);
                    return h.tagName = d.tagName || "span", h.properties = {
                        ...h.properties,
                        ...f,
                        class: h.properties.class
                    }, d.properties?.class && Gr(h, d.properties.class), h = m(h, _) || h, h;
                }
                const l = [], u = r.decorations.sort((h, d)=>d.start.offset - h.start.offset);
                for (const h of u){
                    const { start: d, end: _ } = h;
                    if (d.line === _.line) o(d.line, d.character, _.character, h);
                    else if (d.line < _.line) {
                        o(d.line, d.character, Number.POSITIVE_INFINITY, h);
                        for(let f = d.line + 1; f < _.line; f++)l.unshift(()=>s(f, h));
                        o(_.line, 0, _.character, h);
                    }
                }
                l.forEach((h)=>h());
            }
        };
    }
    function Ol(t) {
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
    function Wr(t) {
        return t.type === "text" ? t.value : t.type === "element" ? t.children.map(Wr).join("") : "";
    }
    const xl = [
        Il()
    ];
    function bt(t) {
        return [
            ...t.transformers || [],
            ...xl
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
    ], Ut = {
        1: "bold",
        2: "dim",
        3: "italic",
        4: "underline",
        7: "reverse",
        9: "strikethrough"
    };
    function Dl(t, e) {
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
    function zn(t, e) {
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
    function Nl(t) {
        const e = [];
        for(let n = 0; n < t.length; n++){
            const r = t[n], i = Number.parseInt(r);
            if (!Number.isNaN(i)) if (i === 0) e.push({
                type: "resetAll"
            });
            else if (i <= 9) Ut[i] && e.push({
                type: "setDecoration",
                value: Ut[i]
            });
            else if (i <= 29) {
                const o = Ut[i - 20];
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
                const [o, s] = zn(t, n);
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
                const [o, s] = zn(t, n);
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
    function Vl() {
        let t = null, e = null, n = new Set;
        return {
            parse (r) {
                const i = [];
                let o = 0;
                do {
                    const s = Dl(r, o), a = s.sequence ? r.substring(o, s.startPosition) : r.substring(o);
                    if (a.length > 0 && i.push({
                        value: a,
                        foreground: t,
                        background: e,
                        decorations: new Set(n)
                    }), s.sequence) {
                        const l = Nl(s.sequence);
                        for (const u of l)u.type === "resetAll" ? (t = null, e = null, n.clear()) : u.type === "resetForegroundColor" ? t = null : u.type === "resetBackgroundColor" ? e = null : u.type === "resetDecoration" && n.delete(u.value);
                        for (const u of l)u.type === "setForegroundColor" ? t = u.value : u.type === "setBackgroundColor" ? e = u.value : u.type === "setDecoration" && n.add(u.value);
                    }
                    o = s.position;
                }while (o < r.length);
                return i;
            }
        };
    }
    var Ml = {
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
    function $l(t = Ml) {
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
            for(let u = 0; u < ge.length; u++)r.push(e(ge[u]));
            let a = [
                0,
                95,
                135,
                175,
                215,
                255
            ];
            for(let u = 0; u < 6; u++)for(let h = 0; h < 6; h++)for(let d = 0; d < 6; d++)r.push(n([
                a[u],
                a[h],
                a[d]
            ]));
            let l = 8;
            for(let u = 0; u < 24; u++, l += 10)r.push(n([
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
    function jl(t, e, n) {
        const r = yt(t, n), i = Lt(e), o = $l(Object.fromEntries(ge.map((a)=>[
                a,
                t.colors?.[`terminal.ansi${a[0].toUpperCase()}${a.substring(1)}`]
            ]))), s = Vl();
        return i.map((a)=>s.parse(a[0]).map((l)=>{
                let u, h;
                l.decorations.has("reverse") ? (u = l.background ? o.value(l.background) : t.bg, h = l.foreground ? o.value(l.foreground) : t.fg) : (u = l.foreground ? o.value(l.foreground) : t.fg, h = l.background ? o.value(l.background) : void 0), u = _e(u, r), h = _e(h, r), l.decorations.has("dim") && (u = Bl(u));
                let d = ae.None;
                return l.decorations.has("bold") && (d |= ae.Bold), l.decorations.has("italic") && (d |= ae.Italic), l.decorations.has("underline") && (d |= ae.Underline), {
                    content: l.value,
                    offset: a[1],
                    color: u,
                    bgColor: h,
                    fontStyle: d
                };
            }));
    }
    function Bl(t) {
        const e = t.match(/#([0-9a-f]{3})([0-9a-f]{3})?([0-9a-f]{2})?/);
        if (e) if (e[3]) {
            const r = Math.round(Number.parseInt(e[3], 16) / 2).toString(16).padStart(2, "0");
            return `#${e[1]}${e[2]}${r}`;
        } else return e[2] ? `#${e[1]}${e[2]}80` : `#${Array.from(e[1]).map((r)=>`${r}${r}`).join("")}80`;
        const n = t.match(/var\((--[\w-]+-ansi-[\w-]+)\)/);
        return n ? `var(${n[1]}-dim)` : t;
    }
    function mn(t, e, n = {}) {
        const { lang: r = "text", theme: i = t.getLoadedThemes()[0] } = n;
        if (dn(r) || hn(i)) return Lt(e).map((l)=>[
                {
                    content: l[0],
                    offset: l[1]
                }
            ]);
        const { theme: o, colorMap: s } = t.setTheme(i);
        if (r === "ansi") return jl(o, e, n);
        const a = t.getLanguage(r);
        if (n.grammarState) {
            if (n.grammarState.lang !== a.name) throw new ce(`Grammar state language "${n.grammarState.lang}" does not match highlight language "${a.name}"`);
            if (!n.grammarState.themes.includes(o.name)) throw new ce(`Grammar state themes "${n.grammarState.themes}" do not contain highlight theme "${o.name}"`);
        }
        return Ul(e, a, o, s, n);
    }
    function Gl(...t) {
        if (t.length === 2) return Be(t[1]);
        const [e, n, r = {}] = t, { lang: i = "text", theme: o = e.getLoadedThemes()[0] } = r;
        if (dn(i) || hn(o)) throw new ce("Plain language does not have grammar state");
        if (i === "ansi") throw new ce("ANSI language does not have grammar state");
        const { theme: s, colorMap: a } = e.setTheme(o), l = e.getLanguage(i);
        return new Pe(Et(n, l, s, a, r).stateStack, l.name, s.name);
    }
    function Ul(t, e, n, r, i) {
        const o = Et(t, e, n, r, i), s = new Pe(Et(t, e, n, r, i).stateStack, e.name, n.name);
        return Pt(o.tokens, s), o.tokens;
    }
    function Et(t, e, n, r, i) {
        const o = yt(n, i), { tokenizeMaxLineLength: s = 0, tokenizeTimeLimit: a = 500 } = i, l = Lt(t);
        let u = i.grammarState ? Cl(i.grammarState, n.name) ?? Qt : i.grammarContextCode != null ? Et(i.grammarContextCode, e, n, r, {
            ...i,
            grammarState: void 0,
            grammarContextCode: void 0
        }).stateStack : Qt, h = [];
        const d = [];
        for(let _ = 0, f = l.length; _ < f; _++){
            const [m, w] = l[_];
            if (m === "") {
                h = [], d.push([]);
                continue;
            }
            if (s > 0 && m.length >= s) {
                h = [], d.push([
                    {
                        content: m,
                        offset: w,
                        color: "",
                        fontStyle: 0
                    }
                ]);
                continue;
            }
            let g, E, p;
            i.includeExplanation && (g = e.tokenizeLine(m, u), E = g.tokens, p = 0);
            const b = e.tokenizeLine2(m, u, a), v = b.tokens.length / 2;
            for(let T = 0; T < v; T++){
                const O = b.tokens[2 * T], V = T + 1 < v ? b.tokens[2 * T + 2] : m.length;
                if (O === V) continue;
                const Z = b.tokens[2 * T + 1], oe = _e(r[Re.getForeground(Z)], o), ue = Re.getFontStyle(Z), de = {
                    content: m.substring(O, V),
                    offset: w + O,
                    color: oe,
                    fontStyle: ue
                };
                if (i.includeExplanation) {
                    const Ee = [];
                    if (i.includeExplanation !== "scopeName") for (const z of n.settings){
                        let X;
                        switch(typeof z.scope){
                            case "string":
                                X = z.scope.split(/,/).map((we)=>we.trim());
                                break;
                            case "object":
                                X = z.scope;
                                break;
                            default:
                                continue;
                        }
                        Ee.push({
                            settings: z,
                            selectors: X.map((we)=>we.split(/ /))
                        });
                    }
                    de.explanation = [];
                    let J = 0;
                    for(; O + J < V;){
                        const z = E[p], X = m.substring(z.startIndex, z.endIndex);
                        J += X.length, de.explanation.push({
                            content: X,
                            scopes: i.includeExplanation === "scopeName" ? Hl(z.scopes) : Fl(Ee, z.scopes)
                        }), p += 1;
                    }
                }
                h.push(de);
            }
            d.push(h), h = [], u = b.ruleStack;
        }
        return {
            tokens: d,
            stateStack: u
        };
    }
    function Hl(t) {
        return t.map((e)=>({
                scopeName: e
            }));
    }
    function Fl(t, e) {
        const n = [];
        for(let r = 0, i = e.length; r < i; r++){
            const o = e[r];
            n[r] = {
                scopeName: o,
                themeMatches: zl(t, o, e.slice(0, r))
            };
        }
        return n;
    }
    function qn(t, e) {
        return t === e || e.substring(0, t.length) === t && e[t.length] === ".";
    }
    function Wl(t, e, n) {
        if (!qn(t[t.length - 1], e)) return !1;
        let r = t.length - 2, i = n.length - 1;
        for(; r >= 0 && i >= 0;)qn(t[r], n[i]) && (r -= 1), i -= 1;
        return r === -1;
    }
    function zl(t, e, n) {
        const r = [];
        for (const { selectors: i, settings: o } of t)for (const s of i)if (Wl(s, e, n)) {
            r.push(o);
            break;
        }
        return r;
    }
    function zr(t, e, n) {
        const r = Object.entries(n.themes).filter((l)=>l[1]).map((l)=>({
                color: l[0],
                theme: l[1]
            })), i = r.map((l)=>{
            const u = mn(t, e, {
                ...n,
                theme: l.theme
            }), h = Be(u), d = typeof l.theme == "string" ? l.theme : l.theme.name;
            return {
                tokens: u,
                state: h,
                theme: d
            };
        }), o = ql(...i.map((l)=>l.tokens)), s = o[0].map((l, u)=>l.map((h, d)=>{
                const _ = {
                    content: h.content,
                    variants: {},
                    offset: h.offset
                };
                return "includeExplanation" in n && n.includeExplanation && (_.explanation = h.explanation), o.forEach((f, m)=>{
                    const { content: w, explanation: g, offset: E, ...p } = f[u][d];
                    _.variants[r[m].color] = p;
                }), _;
            })), a = i[0].state ? new Pe(Object.fromEntries(i.map((l)=>[
                l.theme,
                l.state?.getInternalStack(l.theme)
            ])), i[0].state.lang) : void 0;
        return a && Pt(s, a), s;
    }
    function ql(...t) {
        const e = t.map(()=>[]), n = t.length;
        for(let r = 0; r < t[0].length; r++){
            const i = t.map((l)=>l[r]), o = e.map(()=>[]);
            e.forEach((l, u)=>l.push(o[u]));
            const s = i.map(()=>0), a = i.map((l)=>l[0]);
            for(; a.every((l)=>l);){
                const l = Math.min(...a.map((u)=>u.content.length));
                for(let u = 0; u < n; u++){
                    const h = a[u];
                    h.content.length === l ? (o[u].push(h), s[u] += 1, a[u] = i[u][s[u]]) : (o[u].push({
                        ...h,
                        content: h.content.slice(0, l)
                    }), a[u] = {
                        ...h,
                        content: h.content.slice(l),
                        offset: h.offset + l
                    });
                }
            }
        }
        return e;
    }
    function wt(t, e, n) {
        let r, i, o, s, a, l;
        if ("themes" in n) {
            const { defaultColor: u = "light", cssVariablePrefix: h = "--shiki-" } = n, d = Object.entries(n.themes).filter((g)=>g[1]).map((g)=>({
                    color: g[0],
                    theme: g[1]
                })).sort((g, E)=>g.color === u ? -1 : E.color === u ? 1 : 0);
            if (d.length === 0) throw new ce("`themes` option must not be empty");
            const _ = zr(t, e, n);
            if (l = Be(_), u && !d.find((g)=>g.color === u)) throw new ce(`\`themes\` option must contain the defaultColor key \`${u}\``);
            const f = d.map((g)=>t.getTheme(g.theme)), m = d.map((g)=>g.color);
            o = _.map((g)=>g.map((E)=>Kl(E, m, h, u))), l && Pt(o, l);
            const w = d.map((g)=>yt(g.theme, n));
            i = d.map((g, E)=>(E === 0 && u ? "" : `${h + g.color}:`) + (_e(f[E].fg, w[E]) || "inherit")).join(";"), r = d.map((g, E)=>(E === 0 && u ? "" : `${h + g.color}-bg:`) + (_e(f[E].bg, w[E]) || "inherit")).join(";"), s = `shiki-themes ${f.map((g)=>g.name).join(" ")}`, a = u ? void 0 : [
                i,
                r
            ].join(";");
        } else if ("theme" in n) {
            const u = yt(n.theme, n);
            o = mn(t, e, n);
            const h = t.getTheme(n.theme);
            r = _e(h.bg, u), i = _e(h.fg, u), s = h.name, l = Be(o);
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
    function Kl(t, e, n, r) {
        const i = {
            content: t.content,
            explanation: t.explanation,
            offset: t.offset
        }, o = e.map((l)=>Hr(t.variants[l])), s = new Set(o.flatMap((l)=>Object.keys(l))), a = {};
        return o.forEach((l, u)=>{
            for (const h of s){
                const d = l[h] || "inherit";
                if (u === 0 && r) a[h] = d;
                else {
                    const _ = h === "color" ? "" : h === "background-color" ? "-bg" : `-${h}`, f = n + e[u] + (h === "color" ? "" : _);
                    a[f] = d;
                }
            }
        }), i.htmlStyle = a, i;
    }
    function vt(t, e, n, r = {
        meta: {},
        options: n,
        codeToHast: (i, o)=>vt(t, i, o),
        codeToTokens: (i, o)=>wt(t, i, o)
    }) {
        let i = e;
        for (const f of bt(n))i = f.preprocess?.call(r, i, n) || i;
        let { tokens: o, fg: s, bg: a, themeName: l, rootStyle: u, grammarState: h } = wt(t, i, n);
        const { mergeWhitespaces: d = !0 } = n;
        d === !0 ? o = Xl(o) : d === "never" && (o = Yl(o));
        const _ = {
            ...r,
            get source () {
                return i;
            }
        };
        for (const f of bt(n))o = f.tokens?.call(_, o) || o;
        return Jl(o, {
            ...n,
            fg: s,
            bg: a,
            themeName: l,
            rootStyle: u
        }, _, h);
    }
    function Jl(t, e, n, r = Be(t)) {
        const i = bt(e), o = [], s = {
            type: "root",
            children: []
        }, { structure: a = "classic", tabindex: l = "0" } = e;
        let u = {
            type: "element",
            tagName: "pre",
            properties: {
                class: `shiki ${e.themeName || ""}`,
                style: e.rootStyle || `background-color:${e.bg};color:${e.fg}`,
                ...l !== !1 && l != null ? {
                    tabindex: l.toString()
                } : {},
                ...Object.fromEntries(Array.from(Object.entries(e.meta || {})).filter(([m])=>!m.startsWith("_")))
            },
            children: []
        }, h = {
            type: "element",
            tagName: "code",
            properties: {},
            children: o
        };
        const d = [], _ = {
            ...n,
            structure: a,
            addClassToHast: Gr,
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
                return u;
            },
            get code () {
                return h;
            },
            get lines () {
                return d;
            }
        };
        if (t.forEach((m, w)=>{
            w && (a === "inline" ? s.children.push({
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
            }, E = 0;
            for (const p of m){
                let b = {
                    type: "element",
                    tagName: "span",
                    properties: {
                        ...p.htmlAttrs
                    },
                    children: [
                        {
                            type: "text",
                            value: p.content
                        }
                    ]
                };
                p.htmlStyle;
                const v = Pl(p.htmlStyle || Hr(p));
                v && (b.properties.style = v);
                for (const T of i)b = T?.span?.call(_, b, w + 1, E, g, p) || b;
                a === "inline" ? s.children.push(b) : a === "classic" && g.children.push(b), E += p.content.length;
            }
            if (a === "classic") {
                for (const p of i)g = p?.line?.call(_, g, w + 1) || g;
                d.push(g), o.push(g);
            }
        }), a === "classic") {
            for (const m of i)h = m?.code?.call(_, h) || h;
            u.children.push(h);
            for (const m of i)u = m?.pre?.call(_, u) || u;
            s.children.push(u);
        }
        let f = s;
        for (const m of i)f = m?.root?.call(_, f) || f;
        return r && Pt(f, r), f;
    }
    function Xl(t) {
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
    function Yl(t) {
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
    function Ql(t, e, n) {
        const r = {
            meta: {},
            options: n,
            codeToHast: (o, s)=>vt(t, o, s),
            codeToTokens: (o, s)=>wt(t, o, s)
        };
        let i = vl(vt(t, e, n, r));
        for (const o of bt(n))i = o.postprocess?.call(r, i, n) || i;
        return i;
    }
    const Kn = {
        light: "#333333",
        dark: "#bbbbbb"
    }, Jn = {
        light: "#fffffe",
        dark: "#1e1e1e"
    }, Xn = "__shiki_resolved";
    function fn(t) {
        if (t?.[Xn]) return t;
        const e = {
            ...t
        };
        e.tokenColors && !e.settings && (e.settings = e.tokenColors, delete e.tokenColors), e.type ||= "dark", e.colorReplacements = {
            ...e.colorReplacements
        }, e.settings ||= [];
        let { bg: n, fg: r } = e;
        if (!n || !r) {
            const a = e.settings ? e.settings.find((l)=>!l.name && !l.scope) : void 0;
            a?.settings?.foreground && (r = a.settings.foreground), a?.settings?.background && (n = a.settings.background), !r && e?.colors?.["editor.foreground"] && (r = e.colors["editor.foreground"]), !n && e?.colors?.["editor.background"] && (n = e.colors["editor.background"]), r || (r = e.type === "light" ? Kn.light : Kn.dark), n || (n = e.type === "light" ? Jn.light : Jn.dark), e.fg = r, e.bg = n;
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
            const l = a.settings?.foreground && !a.settings.foreground.startsWith("#"), u = a.settings?.background && !a.settings.background.startsWith("#");
            if (!l && !u) return a;
            const h = {
                ...a,
                settings: {
                    ...a.settings
                }
            };
            if (l) {
                const d = s(a.settings.foreground);
                e.colorReplacements[d] = a.settings.foreground, h.settings.foreground = d;
            }
            if (u) {
                const d = s(a.settings.background);
                e.colorReplacements[d] = a.settings.background, h.settings.background = d;
            }
            return h;
        });
        for (const a of Object.keys(e.colors || {}))if ((a === "editor.foreground" || a === "editor.background" || a.startsWith("terminal.ansi")) && !e.colors[a]?.startsWith("#")) {
            const l = s(e.colors[a]);
            e.colorReplacements[l] = e.colors[a], e.colors[a] = l;
        }
        return Object.defineProperty(e, Xn, {
            enumerable: !1,
            writable: !1,
            value: !0
        }), e;
    }
    async function qr(t) {
        return Array.from(new Set((await Promise.all(t.filter((e)=>!jr(e)).map(async (e)=>await Ur(e).then((n)=>Array.isArray(n) ? n : [
                    n
                ])))).flat()));
    }
    async function Kr(t) {
        return (await Promise.all(t.map(async (n)=>Br(n) ? null : fn(await Ur(n))))).filter((n)=>!!n);
    }
    class Zl extends la {
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
            const n = fn(e);
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
    class ec {
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
    let Ce = 0;
    function tc(t) {
        Ce += 1, t.warnings !== !1 && Ce >= 10 && Ce % 10 === 0 && console.warn(`[Shiki] ${Ce} instances have been created. Shiki is supposed to be used as a singleton, consider refactoring your code to cache your highlighter instance; Or call \`highlighter.dispose()\` to release unused instances.`);
        let e = !1;
        if (!t.engine) throw new W("`engine` option is required for synchronous mode");
        const n = (t.langs || []).flat(1), r = (t.themes || []).flat(1).map(fn), i = new ec(t.engine, n), o = new Zl(i, r, n, t.langAlias);
        let s;
        function a(p) {
            g();
            const b = o.getGrammar(typeof p == "string" ? p : p.name);
            if (!b) throw new W(`Language \`${p}\` not found, you may need to load it first`);
            return b;
        }
        function l(p) {
            if (p === "none") return {
                bg: "",
                fg: "",
                name: "none",
                settings: [],
                type: "dark"
            };
            g();
            const b = o.getTheme(p);
            if (!b) throw new W(`Theme \`${p}\` not found, you may need to load it first`);
            return b;
        }
        function u(p) {
            g();
            const b = l(p);
            s !== p && (o.setTheme(b), s = p);
            const v = o.getColorMap();
            return {
                theme: b,
                colorMap: v
            };
        }
        function h() {
            return g(), o.getLoadedThemes();
        }
        function d() {
            return g(), o.getLoadedLanguages();
        }
        function _(...p) {
            g(), o.loadLanguages(p.flat(1));
        }
        async function f(...p) {
            return _(await qr(p));
        }
        function m(...p) {
            g();
            for (const b of p.flat(1))o.loadTheme(b);
        }
        async function w(...p) {
            return g(), m(await Kr(p));
        }
        function g() {
            if (e) throw new W("Shiki instance has been disposed");
        }
        function E() {
            e || (e = !0, o.dispose(), Ce -= 1);
        }
        return {
            setTheme: u,
            getTheme: l,
            getLanguage: a,
            getLoadedThemes: h,
            getLoadedLanguages: d,
            loadLanguage: f,
            loadLanguageSync: _,
            loadTheme: w,
            loadThemeSync: m,
            dispose: E,
            [Symbol.dispose]: E
        };
    }
    async function nc(t = {}) {
        t.loadWasm;
        const [e, n, r] = await Promise.all([
            Kr(t.themes || []),
            qr(t.langs || []),
            t.engine || ur(t.loadWasm || vs())
        ]);
        return tc({
            ...t,
            themes: e,
            langs: n,
            engine: r
        });
    }
    async function rc(t = {}) {
        const e = await nc(t);
        return {
            getLastGrammarState: (...n)=>Gl(e, ...n),
            codeToTokensBase: (n, r)=>mn(e, n, r),
            codeToTokensWithThemes: (n, r)=>zr(e, n, r),
            codeToTokens: (n, r)=>wt(e, n, r),
            codeToHast: (n, r)=>vt(e, n, r),
            codeToHtml: (n, r)=>Ql(e, n, r),
            ...e,
            getInternalContext: ()=>e
        };
    }
    function ic(t, e, n) {
        let r, i, o;
        {
            const a = t;
            r = a.langs, i = a.themes, o = a.engine;
        }
        async function s(a) {
            function l(f) {
                if (typeof f == "string") {
                    if (jr(f)) return [];
                    const m = r[f];
                    if (!m) throw new ce(`Language \`${f}\` is not included in this bundle. You may want to load it from external source.`);
                    return m;
                }
                return f;
            }
            function u(f) {
                if (Br(f)) return "none";
                if (typeof f == "string") {
                    const m = i[f];
                    if (!m) throw new ce(`Theme \`${f}\` is not included in this bundle. You may want to load it from external source.`);
                    return m;
                }
                return f;
            }
            const h = (a.themes ?? []).map((f)=>u(f)), d = (a.langs ?? []).map((f)=>l(f)), _ = await rc({
                engine: a.engine ?? o(),
                ...a,
                themes: h,
                langs: d
            });
            return {
                ..._,
                loadLanguage (...f) {
                    return _.loadLanguage(...f.map(l));
                },
                loadTheme (...f) {
                    return _.loadTheme(...f.map(u));
                }
            };
        }
        return s;
    }
    function oc(t) {
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
    function sc(t) {
        const e = oc(t);
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
    const ac = ic({
        langs: ns,
        themes: is,
        engine: ()=>ur(c(()=>import("./wasm-CG6Dc4jp.js"), []))
    }), { codeToHtml: lc } = sc(ac);
    var cc = N("<div class=html-viewer-container>");
    function uc(t) {
        const [e, n] = ee(""), r = (i)=>{
            let o = "", s = 0;
            const a = "  ";
            return i.split(/(<[^>]+>)/g).filter((u)=>u.trim()).forEach((u)=>{
                if (u.startsWith("</")) s = Math.max(0, s - 1), o += a.repeat(s) + u + `
`;
                else if (u.startsWith("<")) {
                    const h = u.endsWith("/>") || /^<(area|base|br|col|embed|hr|img|input|link|meta|param|source|track|wbr)/.test(u);
                    o += a.repeat(s) + u + `
`, h || s++;
                } else {
                    const h = u.trim();
                    h && (o += a.repeat(s) + h + `
`);
                }
            }), o.trimEnd();
        };
        return er(async ()=>{
            const i = r(t.html);
            try {
                const o = await lc(i, {
                    lang: "html",
                    theme: "github-dark"
                });
                n(o);
            } catch (o) {
                console.error("Shiki highlighting error:", o), n(`<pre><code>${i}</code></pre>`);
            }
        }), (()=>{
            var i = cc();
            return re(()=>i.innerHTML = e()), i;
        })();
    }
    var dc = N('<div class="border-b border-gray-200 bg-white px-6 py-4"><h2 class="mb-3 text-sm font-semibold text-gray-700">Parser Options</h2><div class="grid grid-cols-2 gap-3 md:grid-cols-4">'), Yn = N('<div class="json-viewer-container flex-1 overflow-auto rounded-lg border border-gray-300 bg-gray-900 p-4">'), hc = N('<div class="flex-1 overflow-auto rounded-lg border border-gray-300 bg-gray-900">'), mc = N('<div class="markdown-preview flex-1 overflow-auto rounded-lg border border-gray-300 bg-white p-6">'), fc = N('<main class="flex h-full w-full flex-col bg-gray-50"><header class="border-b border-gray-200 bg-white px-6 py-4 shadow-sm"><div class="flex items-center justify-between"><div><h1 class="text-2xl font-bold text-gray-900">Markdown Parser Playground</h1><p class="text-sm text-gray-500">Test and explore markdown parsing with various options</p></div><button class="rounded-lg border border-gray-300 bg-white px-4 py-2 text-sm font-medium text-gray-700 hover:bg-gray-50"> Options</button></div></header><div class="flex flex-1 gap-4 overflow-hidden p-6"><div class="flex flex-1 flex-col"><div class="mb-2 flex items-center justify-between"><label for=input class="text-sm font-semibold text-gray-700">Input</label><button class="rounded px-2 py-1 text-xs text-gray-500 hover:bg-gray-100">Clear</button></div><textarea id=input class="flex-1 resize-none rounded-lg border border-gray-300 bg-white p-4 font-mono text-sm outline-none focus:border-blue-500 focus:ring-2 focus:ring-blue-200"placeholder="Enter markdown text here..."></textarea></div><div class="flex flex-1 flex-col"><div class="mb-2 flex items-center justify-between"><div class="flex items-center gap-3"><span class="text-sm font-semibold text-gray-700">Output</span><span class="rounded-full bg-blue-100 px-2.5 py-0.5 text-xs font-semibold text-blue-800">ms</span></div><button class="rounded px-2 py-1 text-xs text-gray-500 hover:bg-gray-100">Copy</button></div><div class="mb-3 flex gap-1"><button><svg class="h-3.5 w-3.5"fill=none stroke=currentColor viewBox="0 0 24 24"><path stroke-linecap=round stroke-linejoin=round stroke-width=2 d="M10 20l4-16m4 4l4 4-4 4M6 16l-4-4 4-4"></path></svg>AST</button><button><svg class="h-3.5 w-3.5"fill=none stroke=currentColor viewBox="0 0 24 24"><path stroke-linecap=round stroke-linejoin=round stroke-width=2 d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z"></path></svg>Frontmatter</button><button><svg class="h-3.5 w-3.5"fill=none stroke=currentColor viewBox="0 0 24 24"><path stroke-linecap=round stroke-linejoin=round stroke-width=2 d="M7 21h10a2 2 0 002-2V9.414a1 1 0 00-.293-.707l-5.414-5.414A1 1 0 0012.586 3H7a2 2 0 00-2 2v14a2 2 0 002 2z"></path></svg>HTML</button><button><svg class="h-3.5 w-3.5"fill=none stroke=currentColor viewBox="0 0 24 24"><path stroke-linecap=round stroke-linejoin=round stroke-width=2 d="M15 12a3 3 0 11-6 0 3 3 0 016 0z"></path><path stroke-linecap=round stroke-linejoin=round stroke-width=2 d="M2.458 12C3.732 7.943 7.523 5 12 5c4.478 0 8.268 2.943 9.542 7-1.274 4.057-5.064 7-9.542 7-4.477 0-8.268-2.943-9.542-7z"></path></svg>Preview'), pc = N('<label class="flex cursor-pointer items-start gap-2"><input type=checkbox class="mt-0.5 h-4 w-4 rounded border-gray-300 text-blue-600 focus:ring-2 focus:ring-blue-500"><div class=flex-1><div class="text-sm font-medium text-gray-700">'), _c = N('<div class="text-xs text-gray-500">'), gc = N('<div class="flex h-full items-center justify-center text-gray-500"><div class=text-center><svg class="mx-auto h-12 w-12 text-gray-400"fill=none stroke=currentColor viewBox="0 0 24 24"><path stroke-linecap=round stroke-linejoin=round stroke-width=2 d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z"></path></svg><p class="mt-2 text-sm">No frontmatter found</p><p class="mt-1 text-xs text-gray-600">Add YAML frontmatter at the top of your markdown');
    const yc = `---
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
`, bc = [
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
            key: "mdx_component",
            label: "MDX Component",
            description: "Support MDX components"
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
    function Ec() {
        const [t, e] = ee(yc), [n, r] = ee(null), [i, o] = ee(""), [s, a] = ee(null), [l, u] = ee(), [h, d] = ee(!1), [_, f] = ee("ast"), [m, w] = ee({
            github_flavored: !0,
            obsidian_flavored: !0,
            cjk_autocorrect: !0
        });
        let g;
        const E = (b)=>{
            w((v)=>({
                    ...v,
                    [b]: !v[b]
                }));
        };
        er(()=>{
            const b = t(), v = m(), T = performance.now(), O = Li(b, v);
            u(Math.ceil((performance.now() - T) * 100) / 100), r(O.tree), o(O.to_html());
            const V = O.frontmatter;
            if (V instanceof Map) {
                const Z = {};
                V.forEach((oe, ue)=>{
                    Z[ue] = oe;
                }), a(Z);
            } else a(V);
        });
        const p = (b, v)=>{
            if (!g || !b || !v) return;
            const O = t().split(`
`);
            let V = 0;
            for(let J = 0; J < b.line - 1; J++)V += O[J].length + 1;
            V += b.column - 1;
            let Z = 0;
            for(let J = 0; J < v.line - 1; J++)Z += O[J].length + 1;
            Z += v.column - 1, g.focus(), g.setSelectionRange(V, Z);
            const oe = parseInt(getComputedStyle(g).lineHeight) || 20, ue = (b.line - 1) * oe, de = g.clientHeight, Ee = ue - de / 2 + oe / 2;
            g.scrollTo({
                top: Math.max(0, Ee),
                behavior: "smooth"
            });
        };
        return (()=>{
            var b = fc(), v = b.firstChild, T = v.firstChild, O = T.firstChild, V = O.nextSibling, Z = V.firstChild, oe = v.nextSibling, ue = oe.firstChild, de = ue.firstChild, Ee = de.firstChild, J = Ee.nextSibling, z = de.nextSibling, X = ue.nextSibling, we = X.firstChild, pn = we.firstChild, Jr = pn.firstChild, _n = Jr.nextSibling, Xr = _n.firstChild, Yr = pn.nextSibling, Qr = we.nextSibling, kt = Qr.firstChild, Ct = kt.nextSibling, It = Ct.nextSibling, gn = It.nextSibling;
            V.$$click = ()=>d(!h()), R(V, ()=>h() ? "Hide" : "Show", Z), R(b, I(G, {
                get when () {
                    return h();
                },
                get children () {
                    var P = dc(), qe = P.firstChild, Ke = qe.nextSibling;
                    return R(Ke, I(sr, {
                        each: bc,
                        children: (he)=>(()=>{
                                var ke = pc(), Ot = ke.firstChild, bn = Ot.nextSibling, Zr = bn.firstChild;
                                return Ot.addEventListener("change", ()=>E(he.key)), R(Zr, ()=>he.label), R(bn, (()=>{
                                    var ei = ie(()=>!!he.description);
                                    return ()=>ei() && (()=>{
                                            var En = _c();
                                            return R(En, ()=>he.description), En;
                                        })();
                                })(), null), re(()=>Ot.checked = !!m()[he.key]), ke;
                            })()
                    })), P;
                }
            }), oe), J.$$click = ()=>e(""), pi(z, "input", (P)=>e(P.currentTarget.value));
            var yn = g;
            return typeof yn == "function" ? _i(yn, z) : g = z, R(_n, l, Xr), Yr.$$click = ()=>{
                const P = _() === "ast" ? JSON.stringify(n(), null, 2) : i();
                navigator.clipboard.writeText(P);
            }, kt.$$click = ()=>f("ast"), Ct.$$click = ()=>f("frontmatter"), It.$$click = ()=>f("html"), gn.$$click = ()=>f("preview"), R(X, I(G, {
                get when () {
                    return _() === "ast";
                },
                get children () {
                    var P = Yn();
                    return R(P, I(G, {
                        get when () {
                            return n();
                        },
                        get children () {
                            return I(Wt, {
                                get data () {
                                    return n();
                                },
                                onNodeClick: p
                            });
                        }
                    })), P;
                }
            }), null), R(X, I(G, {
                get when () {
                    return _() === "frontmatter";
                },
                get children () {
                    var P = Yn();
                    return R(P, I(G, {
                        get when () {
                            return ie(()=>!!(s() && typeof s() == "object"))() && Object.keys(s()).length > 0;
                        },
                        get fallback () {
                            return gc();
                        },
                        get children () {
                            return I(Wt, {
                                get data () {
                                    return s();
                                }
                            });
                        }
                    })), P;
                }
            }), null), R(X, I(G, {
                get when () {
                    return _() === "html";
                },
                get children () {
                    var P = hc();
                    return R(P, I(uc, {
                        get html () {
                            return i();
                        }
                    })), P;
                }
            }), null), R(X, I(G, {
                get when () {
                    return _() === "preview";
                },
                get children () {
                    var P = mc();
                    return re(()=>P.innerHTML = i()), P;
                }
            }), null), re((P)=>{
                var qe = `tab-button ${_() === "ast" ? "tab-button-active" : ""}`, Ke = `tab-button ${_() === "frontmatter" ? "tab-button-active" : ""}`, he = `tab-button ${_() === "html" ? "tab-button-active" : ""}`, ke = `tab-button ${_() === "preview" ? "tab-button-active" : ""}`;
                return qe !== P.e && Ie(kt, P.e = qe), Ke !== P.t && Ie(Ct, P.t = Ke), he !== P.a && Ie(It, P.a = he), ke !== P.o && Ie(gn, P.o = ke), P;
            }, {
                e: void 0,
                t: void 0,
                a: void 0,
                o: void 0
            }), re(()=>z.value = t()), b;
        })();
    }
    ar([
        "click"
    ]);
    const wc = document.getElementById("root");
    fi(()=>I(Ec, {}), wc);
})();

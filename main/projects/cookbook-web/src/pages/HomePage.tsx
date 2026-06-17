import { Link, useNavigate } from '@tanstack/react-router'
import { Search, ArrowRight, Shuffle } from 'lucide-react'
import { useState } from 'react'
import { useCategories, useRecipes } from '../api/hooks'
import { RecipeCard } from '../components/RecipeCard'
import { RecipeCardSkeleton } from '../components/Skeleton'

const HERO_CATEGORIES = ['荤菜', '素菜', '早餐', '汤品', '主食', '甜点']

export function HomePage() {
  const [q, setQ] = useState('')
  const navigate = useNavigate()
  const { data: categories } = useCategories()
  const { data: featured, isLoading } = useRecipes({ per_page: 8 })

  function handleSearch(e: React.FormEvent) {
    e.preventDefault()
    if (q.trim()) navigate({ to: '/search', search: { q } as never })
  }

  return (
    <div>
      {/* ── Hero ──────────────────────────────────────────────────────── */}
      <section
        className="relative overflow-hidden px-8 py-20 md:py-28"
        style={{ background: 'var(--color-ink)' }}
      >
        {/* Decorative grain overlay */}
        <div
          className="absolute inset-0 pointer-events-none opacity-[0.03]"
          style={{
            backgroundImage: `url("data:image/svg+xml,%3Csvg viewBox='0 0 200 200' xmlns='http://www.w3.org/2000/svg'%3E%3Cfilter id='n'%3E%3CfeTurbulence type='fractalNoise' baseFrequency='0.9' numOctaves='4'/%3E%3C/filter%3E%3Crect width='100%25' height='100%25' filter='url(%23n)'/%3E%3C/svg%3E")`,
          }}
        />

        <div className="relative max-w-4xl mx-auto">
          <p
            className="text-xs font-medium tracking-widest uppercase mb-4"
            style={{ color: 'var(--color-gold)' }}
          >
            收录 700+ 家常菜谱
          </p>

          <h1
            className="text-5xl md:text-7xl font-bold leading-none tracking-tight mb-6"
            style={{ fontFamily: 'var(--font-display)', color: 'var(--color-paper)' }}
          >
            今晚<br />
            <span style={{ color: 'var(--color-gold)', fontStyle: 'italic' }}>吃什么？</span>
          </h1>

          <p className="text-base mb-10 max-w-md" style={{ color: 'rgba(250,247,242,0.55)' }}>
            从快手炒菜到功夫炖肉，找到适合今天心情的那道菜。
          </p>

          {/* Search bar */}
          <form onSubmit={handleSearch} className="flex gap-2 max-w-lg">
            <div
              className="flex items-center flex-1 gap-2 px-4 py-3 rounded-xl"
              style={{ background: 'rgba(250,247,242,0.1)', border: '1px solid rgba(250,247,242,0.15)' }}
            >
              <Search size={16} style={{ color: 'rgba(250,247,242,0.4)', flexShrink: 0 }} />
              <input
                value={q}
                onChange={e => setQ(e.target.value)}
                placeholder="搜索菜名、食材…"
                className="flex-1 bg-transparent outline-none text-sm"
                style={{ color: 'var(--color-paper)', caretColor: 'var(--color-gold)' }}
              />
            </div>
            <button
              type="submit"
              className="px-5 py-3 rounded-xl text-sm font-medium transition-opacity hover:opacity-90"
              style={{ background: 'var(--color-rust)', color: 'white' }}
            >
              搜索
            </button>
          </form>

          {/* Category quick links */}
          <div className="flex flex-wrap gap-2 mt-6">
            {HERO_CATEGORIES.map(cat => (
              <Link
                key={cat}
                to="/recipes"
                search={{ category: cat } as never}
                className="text-xs px-3 py-1.5 rounded-full transition-colors hover:opacity-80"
                style={{
                  background: 'rgba(250,247,242,0.08)',
                  border: '1px solid rgba(250,247,242,0.12)',
                  color: 'rgba(250,247,242,0.7)',
                }}
              >
                {cat}
              </Link>
            ))}
          </div>
        </div>
      </section>

      {/* ── All Categories ─────────────────────────────────────────────── */}
      {categories && categories.length > 0 && (
        <section className="px-8 py-10 border-b" style={{ borderColor: 'var(--color-border)' }}>
          <div className="max-w-6xl mx-auto">
            <div className="flex gap-3 flex-wrap">
              {categories.map(c => (
                <Link
                  key={c.name}
                  to="/recipes"
                  search={{ category: c.name } as never}
                  className="flex items-center gap-2 px-4 py-2 rounded-lg text-sm transition-all hover:shadow-sm"
                  style={{
                    background: 'white',
                    border: '1px solid var(--color-border)',
                    color: 'var(--color-ink)',
                  }}
                >
                  <span className="font-medium">{c.name}</span>
                  <span className="text-xs" style={{ color: 'var(--color-ink-muted)' }}>
                    {c.count}
                  </span>
                </Link>
              ))}
            </div>
          </div>
        </section>
      )}

      {/* ── Featured Recipes ───────────────────────────────────────────── */}
      <section className="px-8 py-12">
        <div className="max-w-6xl mx-auto">
          <div className="flex items-center justify-between mb-8">
            <h2
              className="text-2xl font-bold"
              style={{ fontFamily: 'var(--font-display)', color: 'var(--color-ink)' }}
            >
              精选菜谱
            </h2>
            <div className="flex gap-3">
              <Link
                to="/recipes"
                className="flex items-center gap-1.5 text-sm transition-opacity hover:opacity-70"
                style={{ color: 'var(--color-rust)' }}
              >
                查看全部 <ArrowRight size={14} />
              </Link>
            </div>
          </div>

          <div className="grid grid-cols-2 sm:grid-cols-3 lg:grid-cols-4 gap-4">
            {isLoading
              ? Array.from({ length: 8 }, (_, i) => <RecipeCardSkeleton key={i} />)
              : featured?.data.map(r => <RecipeCard key={r.id} recipe={r} />)
            }
          </div>
        </div>
      </section>

      {/* ── Feature Cards ──────────────────────────────────────────────── */}
      <section
        className="px-8 py-12"
        style={{ background: 'var(--color-paper-dark)', borderTop: '1px solid var(--color-border)' }}
      >
        <div className="max-w-6xl mx-auto grid grid-cols-1 md:grid-cols-3 gap-5">
          <FeatureCard
            emoji="🥘"
            title="以材查菜"
            desc="家里有什么食材就搜什么，告别每天纠结吃什么的烦恼。"
            to="/by-ingredients"
            cta="开始查询"
          />
          <FeatureCard
            emoji="📅"
            title="一键生成餐单"
            desc="设置天数和人数，AI 自动帮你安排一周的早午晚三餐。"
            to="/meal-planner"
            cta="生成餐单"
          />
          <FeatureCard
            emoji="📊"
            title="数据统计"
            desc="查看菜谱分类分布、平均热量、难度分布等有趣数据。"
            to="/stats"
            cta="查看统计"
          />
        </div>
      </section>
    </div>
  )
}

function FeatureCard({
  emoji, title, desc, to, cta,
}: {
  emoji: string; title: string; desc: string; to: string; cta: string
}) {
  return (
    <div
      className="flex flex-col gap-4 p-6 rounded-xl"
      style={{ background: 'white', border: '1px solid var(--color-border)' }}
    >
      <span className="text-3xl">{emoji}</span>
      <div>
        <h3
          className="font-semibold text-base mb-1"
          style={{ fontFamily: 'var(--font-display)', color: 'var(--color-ink)' }}
        >
          {title}
        </h3>
        <p className="text-sm leading-relaxed" style={{ color: 'var(--color-ink-muted)' }}>
          {desc}
        </p>
      </div>
      <Link
        to={to}
        className="mt-auto flex items-center gap-1.5 text-sm font-medium transition-opacity hover:opacity-70"
        style={{ color: 'var(--color-rust)' }}
      >
        {cta} <ArrowRight size={13} />
      </Link>
    </div>
  )
}

import { useState, useEffect, useRef } from 'react'
import { Search, X } from 'lucide-react'
import { useRecipeSearch } from '../api/hooks'
import { RecipeCard } from '../components/RecipeCard'
import { RecipeCardSkeleton } from '../components/Skeleton'

function useDebounce<T>(value: T, ms: number): T {
  const [debounced, setDebounced] = useState(value)
  useEffect(() => {
    const t = setTimeout(() => setDebounced(value), ms)
    return () => clearTimeout(t)
  }, [value, ms])
  return debounced
}

export function SearchPage() {
  const [q, setQ] = useState('')
  const [page, setPage] = useState(1)
  const inputRef = useRef<HTMLInputElement>(null)
  const debouncedQ = useDebounce(q, 300)

  const { data, isLoading, isFetching } = useRecipeSearch({
    q: debouncedQ,
    page,
    per_page: 20,
  })

  // Auto-focus on mount
  useEffect(() => { inputRef.current?.focus() }, [])

  // Reset page when query changes
  useEffect(() => { setPage(1) }, [debouncedQ])

  const showResults = debouncedQ.length >= 1
  const isEmpty = showResults && !isLoading && data?.data.length === 0

  return (
    <div className="max-w-5xl mx-auto px-8 py-10">
      {/* Search input */}
      <div className="mb-10">
        <h1
          className="text-3xl font-bold mb-6"
          style={{ fontFamily: 'var(--font-display)', color: 'var(--color-ink)' }}
        >
          搜索菜谱
        </h1>

        <div
          className="flex items-center gap-3 px-5 py-4 rounded-2xl"
          style={{
            background: 'white',
            border: '2px solid var(--color-border)',
            boxShadow: 'var(--shadow-card)',
            transition: 'border-color 0.2s',
          }}
          onFocusCapture={e => (e.currentTarget.style.borderColor = 'var(--color-ink)')}
          onBlurCapture={e => (e.currentTarget.style.borderColor = 'var(--color-border)')}
        >
          <Search size={18} style={{ color: 'var(--color-ink-muted)', flexShrink: 0 }} />
          <input
            ref={inputRef}
            value={q}
            onChange={e => setQ(e.target.value)}
            placeholder="输入菜名、食材或关键词…"
            className="flex-1 bg-transparent outline-none text-base"
            style={{ color: 'var(--color-ink)', caretColor: 'var(--color-rust)' }}
          />
          {q && (
            <button onClick={() => setQ('')}>
              <X size={16} style={{ color: 'var(--color-ink-muted)' }} />
            </button>
          )}
          {isFetching && showResults && (
            <div
              className="w-4 h-4 rounded-full border-2 border-t-transparent animate-spin shrink-0"
              style={{ borderColor: 'var(--color-rust)', borderTopColor: 'transparent' }}
            />
          )}
        </div>

        {showResults && data && (
          <p className="mt-3 text-sm" style={{ color: 'var(--color-ink-muted)' }}>
            找到 <strong style={{ color: 'var(--color-ink)' }}>{data.total}</strong> 个结果
            {debouncedQ.length < 3 && (
              <span className="ml-2 text-xs opacity-60">（输入 3 个字符以上可启用全文搜索）</span>
            )}
          </p>
        )}
      </div>

      {/* Empty prompt */}
      {!showResults && (
        <div className="text-center py-20">
          <p className="text-5xl mb-4">🔍</p>
          <p className="text-base" style={{ color: 'var(--color-ink-muted)' }}>
            试试搜索"红烧肉"、"番茄"或"低卡"
          </p>
        </div>
      )}

      {/* No results */}
      {isEmpty && (
        <div className="text-center py-20">
          <p className="text-5xl mb-4">😶</p>
          <p className="text-base" style={{ color: 'var(--color-ink-muted)' }}>
            没有找到「{debouncedQ}」相关菜谱
          </p>
          <p className="text-sm mt-2 opacity-60" style={{ color: 'var(--color-ink-muted)' }}>
            换个关键词试试？
          </p>
        </div>
      )}

      {/* Results grid */}
      {showResults && (
        <>
          <div className="grid grid-cols-2 sm:grid-cols-3 lg:grid-cols-4 gap-4 mb-10">
            {isLoading
              ? Array.from({ length: 8 }, (_, i) => <RecipeCardSkeleton key={i} />)
              : data?.data.map(r => <RecipeCard key={r.id} recipe={r} />)
            }
          </div>

          {/* Pagination */}
          {data && data.total_pages > 1 && (
            <div className="flex items-center justify-center gap-2 flex-wrap">
              {Array.from({ length: data.total_pages }, (_, i) => i + 1).map(p => (
                <button
                  key={p}
                  onClick={() => setPage(p)}
                  className="w-9 h-9 rounded-lg text-sm font-medium transition-all"
                  style={{
                    background: p === page ? 'var(--color-ink)' : 'white',
                    color: p === page ? 'var(--color-paper)' : 'var(--color-ink-muted)',
                    border: `1px solid ${p === page ? 'var(--color-ink)' : 'var(--color-border)'}`,
                  }}
                >
                  {p}
                </button>
              ))}
            </div>
          )}
        </>
      )}
    </div>
  )
}

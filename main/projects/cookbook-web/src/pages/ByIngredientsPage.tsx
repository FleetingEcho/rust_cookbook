import { useState, useRef, useEffect } from 'react'
import { X, Plus, Salad } from 'lucide-react'
import { useByIngredients, useIngredientSuggest } from '../api/hooks'
import { RecipeCard } from '../components/RecipeCard'
import { RecipeCardSkeleton } from '../components/Skeleton'

function useDebounce<T>(value: T, ms: number): T {
  const [d, setD] = useState(value)
  useEffect(() => {
    const t = setTimeout(() => setD(value), ms)
    return () => clearTimeout(t)
  }, [value, ms])
  return d
}

export function ByIngredientsPage() {
  const [tags, setTags] = useState<string[]>([])
  const [input, setInput] = useState('')
  const [mode, setMode] = useState<'any' | 'all'>('any')
  const [page, setPage] = useState(1)
  const [showSuggestions, setShowSuggestions] = useState(false)
  const inputRef = useRef<HTMLInputElement>(null)
  const debouncedInput = useDebounce(input, 200)

  const { data: suggestions } = useIngredientSuggest({
    q: debouncedInput,
    limit: 8,
  })

  const ingredientsStr = tags.join(',')
  const { data, isLoading } = useByIngredients({
    ingredients: ingredientsStr || undefined,
    match: mode,
    page,
    per_page: 20,
  })

  function addTag(name: string) {
    const trimmed = name.trim()
    if (trimmed && !tags.includes(trimmed)) {
      setTags(prev => [...prev, trimmed])
      setPage(1)
    }
    setInput('')
    setShowSuggestions(false)
    inputRef.current?.focus()
  }

  function removeTag(name: string) {
    setTags(prev => prev.filter(t => t !== name))
    setPage(1)
  }

  function handleKeyDown(e: React.KeyboardEvent) {
    if ((e.key === 'Enter' || e.key === ',') && input.trim()) {
      e.preventDefault()
      addTag(input)
    } else if (e.key === 'Backspace' && !input && tags.length > 0) {
      removeTag(tags[tags.length - 1])
    }
  }

  return (
    <div className="max-w-5xl mx-auto px-8 py-10">
      <div className="mb-8">
        <h1
          className="text-3xl font-bold mb-2"
          style={{ fontFamily: 'var(--font-display)', color: 'var(--color-ink)' }}
        >
          以材查菜
        </h1>
        <p className="text-sm" style={{ color: 'var(--color-ink-muted)' }}>
          输入家里有的食材，找到能做的菜
        </p>
      </div>

      {/* Input area */}
      <div
        className="rounded-2xl p-4 mb-6"
        style={{
          background: 'white',
          border: '1px solid var(--color-border)',
          boxShadow: 'var(--shadow-card)',
        }}
      >
        {/* Tag + input row */}
        <div
          className="flex flex-wrap gap-2 items-center min-h-[40px]"
          onClick={() => inputRef.current?.focus()}
        >
          {tags.map(tag => (
            <span
              key={tag}
              className="flex items-center gap-1.5 px-3 py-1.5 rounded-full text-sm font-medium select-none"
              style={{ background: 'var(--color-ink)', color: 'var(--color-paper)' }}
            >
              {tag}
              <button
                onClick={e => { e.stopPropagation(); removeTag(tag) }}
                className="opacity-60 hover:opacity-100 transition-opacity"
              >
                <X size={12} />
              </button>
            </span>
          ))}

          <div className="relative flex-1 min-w-[120px]">
            <input
              ref={inputRef}
              value={input}
              onChange={e => { setInput(e.target.value); setShowSuggestions(true) }}
              onKeyDown={handleKeyDown}
              onFocus={() => setShowSuggestions(true)}
              onBlur={() => setTimeout(() => setShowSuggestions(false), 150)}
              placeholder={tags.length ? '再加一种…' : '输入食材名，如：豆腐、鸡蛋、土豆…'}
              className="w-full bg-transparent outline-none text-sm py-1"
              style={{ color: 'var(--color-ink)', caretColor: 'var(--color-rust)' }}
            />

            {/* Suggestions dropdown */}
            {showSuggestions && suggestions && suggestions.length > 0 && (
              <div
                className="absolute left-0 top-full mt-1 w-56 rounded-xl overflow-hidden z-20"
                style={{
                  background: 'white',
                  border: '1px solid var(--color-border)',
                  boxShadow: 'var(--shadow-card-hover)',
                }}
              >
                {suggestions.map(s => (
                  <button
                    key={s.name}
                    onMouseDown={() => addTag(s.name)}
                    className="w-full flex items-center justify-between px-4 py-2.5 text-sm text-left hover:bg-[var(--color-paper)] transition-colors"
                    style={{ color: 'var(--color-ink)' }}
                  >
                    <span className="flex items-center gap-2">
                      <Plus size={12} style={{ color: 'var(--color-rust)' }} />
                      {s.name}
                    </span>
                    <span className="text-xs" style={{ color: 'var(--color-ink-muted)' }}>
                      {s.recipe_count} 道菜
                    </span>
                  </button>
                ))}
              </div>
            )}
          </div>
        </div>

        {/* Controls row */}
        {tags.length > 0 && (
          <div
            className="flex items-center justify-between mt-4 pt-3"
            style={{ borderTop: '1px solid var(--color-border)' }}
          >
            <div className="flex items-center gap-2">
              <span className="text-xs" style={{ color: 'var(--color-ink-muted)' }}>匹配方式：</span>
              {(['any', 'all'] as const).map(m => (
                <button
                  key={m}
                  onClick={() => { setMode(m); setPage(1) }}
                  className="text-xs px-3 py-1.5 rounded-full font-medium transition-all"
                  style={{
                    background: mode === m ? 'var(--color-ink)' : 'transparent',
                    color: mode === m ? 'var(--color-paper)' : 'var(--color-ink-muted)',
                    border: `1px solid ${mode === m ? 'var(--color-ink)' : 'var(--color-border)'}`,
                  }}
                >
                  {m === 'any' ? '含任一食材' : '全部包含'}
                </button>
              ))}
            </div>

            <button
              onClick={() => { setTags([]); setPage(1) }}
              className="text-xs transition-opacity hover:opacity-60"
              style={{ color: 'var(--color-ink-muted)' }}
            >
              清空
            </button>
          </div>
        )}
      </div>

      {/* Results count */}
      {tags.length > 0 && data && (
        <p className="text-sm mb-6" style={{ color: 'var(--color-ink-muted)' }}>
          找到 <strong style={{ color: 'var(--color-ink)' }}>{data.total}</strong> 道含
          <strong style={{ color: 'var(--color-ink)' }}>
            {' '}{tags.join('、')}{' '}
          </strong>
          的菜谱
        </p>
      )}

      {/* Empty state */}
      {tags.length === 0 && (
        <div className="text-center py-20">
          <Salad size={48} className="mx-auto mb-4 opacity-20" style={{ color: 'var(--color-ink)' }} />
          <p className="text-base" style={{ color: 'var(--color-ink-muted)' }}>
            添加食材后，自动显示可做的菜
          </p>
          <p className="text-sm mt-1 opacity-60" style={{ color: 'var(--color-ink-muted)' }}>
            按 Enter 或逗号确认每种食材
          </p>
        </div>
      )}

      {/* Grid */}
      {tags.length > 0 && (
        <>
          <div className="grid grid-cols-2 sm:grid-cols-3 lg:grid-cols-4 gap-4 mb-10">
            {isLoading
              ? Array.from({ length: 8 }, (_, i) => <RecipeCardSkeleton key={i} />)
              : data?.data.map(r => <RecipeCard key={r.id} recipe={r} />)
            }
          </div>

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

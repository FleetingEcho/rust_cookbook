import { useSearch, useNavigate } from '@tanstack/react-router'
import { ChevronLeft, ChevronRight, SlidersHorizontal } from 'lucide-react'
import { useRecipes, useCategories } from '../api/hooks'
import { RecipeCard } from '../components/RecipeCard'
import { RecipeCardSkeleton } from '../components/Skeleton'

type RecipesSearch = { category?: string; difficulty?: number; has_image?: boolean; page?: number }

const DIFFICULTIES = [
  { label: '全部', value: undefined },
  { label: '★', value: 1 },
  { label: '★★', value: 2 },
  { label: '★★★', value: 3 },
  { label: '★★★★', value: 4 },
  { label: '★★★★★', value: 5 },
]

export function RecipesPage() {
  const search = useSearch({ strict: false }) as RecipesSearch
  const navigate = useNavigate()

  const category   = search.category
  const difficulty = search.difficulty
  const has_image  = search.has_image
  const page       = search.page ?? 1

  function update(patch: Partial<RecipesSearch>) {
    navigate({
      to: '/recipes',
      search: (prev: RecipesSearch) => ({
        ...prev,
        ...patch,
        // reset page whenever filter changes
        page: patch.page ?? 1,
      }),
    } as never)
  }

  const { data, isLoading } = useRecipes({ page, per_page: 24, category, difficulty, has_image })
  const { data: categories } = useCategories()

  return (
    <div className="max-w-6xl mx-auto px-8 py-10">
      {/* Header */}
      <div className="flex items-center justify-between mb-8">
        <h1
          className="text-3xl font-bold"
          style={{ fontFamily: 'var(--font-display)', color: 'var(--color-ink)' }}
        >
          {category ?? '所有菜谱'}
          {data && (
            <span
              className="ml-3 text-base font-normal"
              style={{ color: 'var(--color-ink-muted)', fontFamily: 'var(--font-body)' }}
            >
              共 {data.total} 道
            </span>
          )}
        </h1>

        {(category || difficulty || has_image !== undefined) && (
          <button
            onClick={() => update({ category: undefined, difficulty: undefined, has_image: undefined })}
            className="flex items-center gap-1.5 text-xs px-3 py-2 rounded-lg transition-opacity hover:opacity-70"
            style={{ color: 'var(--color-ink-muted)', border: '1px solid var(--color-border)', background: 'white' }}
          >
            <SlidersHorizontal size={12} />
            重置筛选
          </button>
        )}
      </div>

      {/* Filters */}
      <div className="flex flex-col gap-4 mb-8">
        {/* Category pills */}
        <div className="flex gap-2 flex-wrap">
          <FilterPill active={!category} onClick={() => update({ category: undefined })}>
            全部分类
          </FilterPill>
          {categories?.map(c => (
            <FilterPill
              key={c.name}
              active={category === c.name}
              onClick={() => update({ category: c.name })}
            >
              {c.name}
              <span className="ml-1 text-[10px] opacity-60">{c.count}</span>
            </FilterPill>
          ))}
        </div>

        {/* Difficulty */}
        <div className="flex gap-2 flex-wrap items-center">
          <span className="text-xs" style={{ color: 'var(--color-ink-muted)' }}>难度：</span>
          {DIFFICULTIES.map(d => (
            <FilterPill
              key={String(d.value)}
              active={difficulty === d.value}
              onClick={() => update({ difficulty: d.value })}
            >
              {d.label}
            </FilterPill>
          ))}
        </div>

        {/* Image filter */}
        <div className="flex gap-2 flex-wrap items-center">
          <span className="text-xs" style={{ color: 'var(--color-ink-muted)' }}>图片：</span>
          <FilterPill active={has_image === undefined} onClick={() => update({ has_image: undefined })}>全部</FilterPill>
          <FilterPill active={has_image === true}      onClick={() => update({ has_image: true })}>有图</FilterPill>
          <FilterPill active={has_image === false}     onClick={() => update({ has_image: false })}>无图</FilterPill>
        </div>
      </div>

      {/* Grid */}
      <div className="grid grid-cols-2 sm:grid-cols-3 lg:grid-cols-4 gap-4 mb-10">
        {isLoading
          ? Array.from({ length: 24 }, (_, i) => <RecipeCardSkeleton key={i} />)
          : data?.data.map(r => <RecipeCard key={r.id} recipe={r} />)
        }
      </div>

      {/* Pagination */}
      {data && data.total_pages > 1 && (
        <div className="flex items-center justify-center gap-3">
          <PageBtn disabled={page <= 1} onClick={() => update({ page: page - 1 })}>
            <ChevronLeft size={14} />
          </PageBtn>
          <span className="text-sm" style={{ color: 'var(--color-ink-muted)' }}>
            第 {page} / {data.total_pages} 页
          </span>
          <PageBtn disabled={page >= data.total_pages} onClick={() => update({ page: page + 1 })}>
            <ChevronRight size={14} />
          </PageBtn>
        </div>
      )}
    </div>
  )
}

function FilterPill({
  children, active, onClick,
}: {
  children: React.ReactNode; active: boolean; onClick: () => void
}) {
  return (
    <button
      onClick={onClick}
      className="text-xs px-3 py-1.5 rounded-full transition-all font-medium"
      style={{
        background: active ? 'var(--color-ink)' : 'white',
        color: active ? 'var(--color-paper)' : 'var(--color-ink-muted)',
        border: `1px solid ${active ? 'var(--color-ink)' : 'var(--color-border)'}`,
      }}
    >
      {children}
    </button>
  )
}

function PageBtn({
  children, disabled, onClick,
}: {
  children: React.ReactNode; disabled: boolean; onClick: () => void
}) {
  return (
    <button
      onClick={onClick}
      disabled={disabled}
      className="w-8 h-8 flex items-center justify-center rounded-lg transition-opacity disabled:opacity-30"
      style={{ background: 'white', border: '1px solid var(--color-border)' }}
    >
      {children}
    </button>
  )
}

import { useParams, Link } from '@tanstack/react-router'
import { ArrowLeft, Flame, ChefHat, CheckCircle2, Circle } from 'lucide-react'
import { useState } from 'react'
import { useRecipe, useSimilarRecipes } from '../api/hooks'
import { RecipeCard } from '../components/RecipeCard'
import { ImageManager } from '../components/ImageManager'
import { Skeleton } from '../components/Skeleton'

// Strip Markdown image syntax left over from seed data: ![alt](path)
function cleanDescription(text: string | null): string | null {
  if (!text) return null
  return text.replace(/!\[.*?\]\(.*?\)\s*/g, '').trim() || null
}

function DifficultyBar({ value }: { value: number | null }) {
  if (!value) return null
  return (
    <div className="flex gap-1">
      {Array.from({ length: 5 }, (_, i) => (
        <div
          key={i}
          className="h-1.5 w-6 rounded-full"
          style={{ background: i < value ? 'var(--color-rust)' : 'var(--color-border)' }}
        />
      ))}
    </div>
  )
}

export function RecipeDetailPage() {
  const { id } = useParams({ from: '/recipes/$id' })
  const recipeId = Number(id)
  const { data: recipe, isLoading, isError } = useRecipe(recipeId)
  const { data: similar } = useSimilarRecipes(recipeId, { limit: 6 })
  const [checked, setChecked] = useState<Set<number>>(new Set())

  function toggleIngredient(idx: number) {
    setChecked(prev => {
      const next = new Set(prev)
      next.has(idx) ? next.delete(idx) : next.add(idx)
      return next
    })
  }

  if (isError) {
    return (
      <div className="max-w-2xl mx-auto px-8 py-20 text-center">
        <p className="text-2xl mb-4">🍽</p>
        <p style={{ color: 'var(--color-ink-muted)' }}>菜谱未找到</p>
        <Link to="/recipes" className="text-sm mt-4 inline-block" style={{ color: 'var(--color-rust)' }}>
          ← 返回菜谱列表
        </Link>
      </div>
    )
  }

  return (
    <div className="max-w-4xl mx-auto px-8 py-10">
      {/* Back */}
      <Link
        to="/recipes"
        className="inline-flex items-center gap-1.5 text-sm mb-8 transition-opacity hover:opacity-70"
        style={{ color: 'var(--color-ink-muted)' }}
      >
        <ArrowLeft size={14} />
        菜谱列表
      </Link>

      {isLoading ? (
        <LoadingSkeleton />
      ) : recipe ? (
        <>
          {/* Hero section */}
          <div className="grid md:grid-cols-2 gap-8 mb-12">
            {/* Cover image with upload/delete management */}
            <div>
              <ImageManager
                recipeId={recipe.id}
                currentUrl={recipe.cover_image}
                recipeName={recipe.name}
              />
            </div>

            {/* Meta */}
            <div className="flex flex-col gap-5">
              <div>
                <span
                  className="text-xs font-medium px-2.5 py-1 rounded-full inline-block mb-3"
                  style={{ background: 'var(--color-rust-light)', color: 'var(--color-rust)' }}
                >
                  {recipe.category}
                </span>
                <h1
                  className="text-3xl font-bold leading-tight"
                  style={{ fontFamily: 'var(--font-display)', color: 'var(--color-ink)' }}
                >
                  {recipe.name}
                </h1>
              </div>

              {cleanDescription(recipe.description) && (
                <p className="text-sm leading-relaxed" style={{ color: 'var(--color-ink-muted)' }}>
                  {cleanDescription(recipe.description)}
                </p>
              )}

              <DifficultyBar value={recipe.difficulty} />

              {/* Stats grid */}
              {recipe.calories && (
                <div className="grid grid-cols-2 gap-3">
                  <StatChip
                    icon={<Flame size={14} style={{ color: 'var(--color-caramel)' }} />}
                    label="热量"
                    value={`${Math.round(recipe.calories)} 大卡`}
                  />
                </div>
              )}

              {/* Tags */}
              {recipe.tags.length > 0 && (
                <div className="flex flex-wrap gap-1.5">
                  {recipe.tags.map(tag => (
                    <span
                      key={tag}
                      className="text-xs px-2.5 py-1 rounded-full"
                      style={{ background: 'var(--color-paper-dark)', color: 'var(--color-ink-muted)' }}
                    >
                      {tag}
                    </span>
                  ))}
                </div>
              )}

              <p className="text-xs" style={{ color: 'var(--color-ink-muted)' }}>
                来源：{recipe.source}
              </p>
            </div>
          </div>

          {/* Ingredients + Steps */}
          <div className="grid md:grid-cols-[280px_1fr] gap-10 mb-14">
            {/* Ingredients */}
            <div>
              <h2
                className="text-lg font-semibold mb-4 flex items-center gap-2"
                style={{ fontFamily: 'var(--font-display)' }}
              >
                <ChefHat size={16} style={{ color: 'var(--color-rust)' }} />
                食材清单
                <span className="text-sm font-normal" style={{ color: 'var(--color-ink-muted)' }}>
                  ({recipe.ingredients.length})
                </span>
              </h2>
              <ul className="flex flex-col gap-1.5">
                {recipe.ingredients.map((ing, idx) => {
                  const done = checked.has(idx)
                  return (
                    <li
                      key={ing.id}
                      onClick={() => toggleIngredient(idx)}
                      className="flex items-center gap-3 py-2 px-3 rounded-lg cursor-pointer transition-colors select-none"
                      style={{
                        background: done ? 'var(--color-paper-dark)' : 'white',
                        border: '1px solid var(--color-border)',
                        opacity: done ? 0.5 : 1,
                      }}
                    >
                      {done
                        ? <CheckCircle2 size={14} style={{ color: 'var(--color-sage)', flexShrink: 0 }} />
                        : <Circle size={14} style={{ color: 'var(--color-border)', flexShrink: 0 }} />
                      }
                      <span className="text-sm flex-1" style={{ color: 'var(--color-ink)' }}>
                        {ing.name}
                      </span>
                      {(ing.amount || ing.unit) && (
                        <span className="text-xs" style={{ color: 'var(--color-ink-muted)' }}>
                          {ing.amount}{ing.unit}
                        </span>
                      )}
                    </li>
                  )
                })}
              </ul>
            </div>

            {/* Steps */}
            <div>
              <h2
                className="text-lg font-semibold mb-6"
                style={{ fontFamily: 'var(--font-display)' }}
              >
                烹饪步骤
              </h2>
              <ol className="flex flex-col gap-6">
                {recipe.steps.map(step => (
                  <li key={step.id} className="flex gap-4">
                    <span
                      className="w-7 h-7 rounded-full flex items-center justify-center text-sm font-bold shrink-0 mt-0.5"
                      style={{
                        background: 'var(--color-ink)',
                        color: 'var(--color-paper)',
                        fontFamily: 'var(--font-display)',
                        fontStyle: 'italic',
                      }}
                    >
                      {step.step_order}
                    </span>
                    <div className="flex-1">
                      <p className="text-sm leading-relaxed" style={{ color: 'var(--color-ink)' }}>
                        {step.content}
                      </p>
                      {step.image_url && (
                        <img
                          src={step.image_url}
                          alt={`步骤 ${step.step_order}`}
                          loading="lazy"
                          className="mt-3 rounded-xl max-h-48 object-cover"
                        />
                      )}
                    </div>
                  </li>
                ))}
              </ol>
            </div>
          </div>

          {/* Nutrition */}
          {recipe.nutrition && (
            <section
              className="rounded-2xl p-6 mb-14"
              style={{ background: 'var(--color-paper-dark)', border: '1px solid var(--color-border)' }}
            >
              <h2
                className="text-lg font-semibold mb-4"
                style={{ fontFamily: 'var(--font-display)' }}
              >
                营养成分（每份）
              </h2>
              <div className="grid grid-cols-2 sm:grid-cols-4 gap-4">
                {[
                  { label: '蛋白质', value: recipe.nutrition.protein_g, unit: 'g' },
                  { label: '脂肪', value: recipe.nutrition.fat_g, unit: 'g' },
                  { label: '碳水化合物', value: recipe.nutrition.carbs_g, unit: 'g' },
                  { label: '钠', value: recipe.nutrition.sodium_mg, unit: 'mg' },
                ].map(n => n.value != null && (
                  <div key={n.label} className="text-center">
                    <div
                      className="text-xl font-bold"
                      style={{ fontFamily: 'var(--font-display)', color: 'var(--color-rust)' }}
                    >
                      {Math.round(n.value)}
                    </div>
                    <div className="text-xs mt-0.5" style={{ color: 'var(--color-ink-muted)' }}>
                      {n.label}
                      <span className="ml-0.5 opacity-60">{n.unit}</span>
                    </div>
                  </div>
                ))}
              </div>
            </section>
          )}

          {/* Similar Recipes */}
          {similar && similar.length > 0 && (
            <section>
              <h2
                className="text-xl font-semibold mb-6"
                style={{ fontFamily: 'var(--font-display)' }}
              >
                相似菜谱
              </h2>
              <div className="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-4 gap-4">
                {similar.map(r => <RecipeCard key={r.id} recipe={r} />)}
              </div>
            </section>
          )}
        </>
      ) : null}
    </div>
  )
}

function StatChip({ icon, label, value }: { icon: React.ReactNode; label: string; value: string }) {
  return (
    <div
      className="flex items-center gap-2 p-3 rounded-xl"
      style={{ background: 'white', border: '1px solid var(--color-border)' }}
    >
      <span style={{ color: 'var(--color-ink-muted)' }}>{icon}</span>
      <div>
        <div className="text-[10px]" style={{ color: 'var(--color-ink-muted)' }}>{label}</div>
        <div className="text-sm font-medium">{value}</div>
      </div>
    </div>
  )
}

function LoadingSkeleton() {
  return (
    <div className="grid md:grid-cols-2 gap-8 mb-12">
      <Skeleton className="aspect-[4/3] rounded-2xl w-full" />
      <div className="flex flex-col gap-4">
        <Skeleton className="h-6 w-24 rounded-full" />
        <Skeleton className="h-9 w-3/4" />
        <Skeleton className="h-4 w-full" />
        <Skeleton className="h-4 w-2/3" />
        <div className="grid grid-cols-2 gap-3">
          {Array.from({ length: 4 }, (_, i) => <Skeleton key={i} className="h-16 rounded-xl" />)}
        </div>
      </div>
    </div>
  )
}

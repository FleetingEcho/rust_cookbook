export function Skeleton({ className = '' }: { className?: string }) {
  return (
    <div
      className={`animate-pulse rounded ${className}`}
      style={{ background: 'var(--color-paper-dark)' }}
    />
  )
}

export function RecipeCardSkeleton() {
  return (
    <div
      className="rounded-xl overflow-hidden"
      style={{ background: 'white', border: '1px solid var(--color-border)' }}
    >
      <Skeleton className="h-44 w-full rounded-none" />
      <div className="p-3.5 flex flex-col gap-2">
        <Skeleton className="h-4 w-3/4" />
        <Skeleton className="h-3 w-1/2" />
      </div>
    </div>
  )
}

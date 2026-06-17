import { useRef, useState } from 'react'
import { Upload, Trash2, X, AlertTriangle, ImagePlus, Loader2 } from 'lucide-react'
import { useUploadImage, useDeleteImage } from '../api/hooks'

interface Props {
  recipeId: number
  currentUrl: string | null
  recipeName: string
}

export function ImageManager({ recipeId, currentUrl, recipeName }: Props) {
  const fileInputRef = useRef<HTMLInputElement>(null)
  const [confirmOpen, setConfirmOpen] = useState(false)
  const [preview, setPreview] = useState<string | null>(null)
  const [dragOver, setDragOver] = useState(false)

  const upload = useUploadImage(recipeId)
  const del = useDeleteImage(recipeId)

  const busy = upload.isPending || del.isPending

  function handleFile(file: File) {
    if (!file.type.startsWith('image/')) return
    // Show local preview immediately
    const url = URL.createObjectURL(file)
    setPreview(url)
    upload.mutate(file, {
      onSettled: () => {
        URL.revokeObjectURL(url)
        setPreview(null)
      },
    })
  }

  function handleInputChange(e: React.ChangeEvent<HTMLInputElement>) {
    const file = e.target.files?.[0]
    if (file) handleFile(file)
    e.target.value = ''
  }

  function handleDrop(e: React.DragEvent) {
    e.preventDefault()
    setDragOver(false)
    const file = e.dataTransfer.files?.[0]
    if (file) handleFile(file)
  }

  function confirmDelete() {
    del.mutate(undefined, { onSettled: () => setConfirmOpen(false) })
  }

  const displayUrl = preview ?? currentUrl

  return (
    <>
      {/* Outer group so hint text can respond to hover on the image area */}
      <div className="group">
      {/* Image area with overlay controls */}
      <div
        className="relative rounded-2xl overflow-hidden aspect-[4/3] group/img"
        style={{ background: 'var(--color-paper-dark)' }}
        onDragOver={e => { e.preventDefault(); setDragOver(true) }}
        onDragLeave={() => setDragOver(false)}
        onDrop={handleDrop}
      >
        {/* Image or placeholder */}
        {displayUrl ? (
          <img
            src={displayUrl}
            alt={recipeName}
            className="w-full h-full object-cover"
            style={{ opacity: busy ? 0.5 : 1, transition: 'opacity 0.2s' }}
          />
        ) : (
          <div className="w-full h-full flex flex-col items-center justify-center gap-3">
            <ImagePlus size={40} style={{ color: 'var(--color-border)' }} />
            <p className="text-sm" style={{ color: 'var(--color-ink-muted)' }}>
              暂无封面图
            </p>
          </div>
        )}

        {/* Drag-over overlay */}
        {dragOver && (
          <div
            className="absolute inset-0 flex items-center justify-center"
            style={{ background: 'rgba(26,20,16,0.6)', backdropFilter: 'blur(2px)' }}
          >
            <div className="text-center">
              <Upload size={32} style={{ color: 'var(--color-gold)', margin: '0 auto 8px' }} />
              <p className="text-sm font-medium" style={{ color: 'var(--color-paper)' }}>
                松开以上传
              </p>
            </div>
          </div>
        )}

        {/* Busy spinner */}
        {busy && (
          <div className="absolute inset-0 flex items-center justify-center"
            style={{ background: 'rgba(26,20,16,0.4)' }}
          >
            <Loader2 size={32} className="animate-spin" style={{ color: 'var(--color-gold)' }} />
          </div>
        )}

        {/* Hover action bar */}
        {!busy && !dragOver && (
          <div
            className="absolute inset-x-0 bottom-0 flex gap-2 p-3 opacity-0 group-hover/img:opacity-100 transition-opacity duration-200"
            style={{ background: 'linear-gradient(to top, rgba(26,20,16,0.85), transparent)' }}
          >
            <button
              onClick={() => fileInputRef.current?.click()}
              className="flex items-center gap-1.5 flex-1 justify-center py-2 rounded-xl text-xs font-medium transition-opacity hover:opacity-80"
              style={{ background: 'var(--color-rust)', color: 'white' }}
            >
              <Upload size={13} />
              {currentUrl ? '替换图片' : '上传图片'}
            </button>

            {currentUrl && (
              <button
                onClick={() => setConfirmOpen(true)}
                className="flex items-center gap-1.5 px-3 py-2 rounded-xl text-xs font-medium transition-opacity hover:opacity-80"
                style={{ background: 'rgba(250,247,242,0.15)', color: 'white', border: '1px solid rgba(250,247,242,0.2)' }}
              >
                <Trash2 size={13} />
                删除
              </button>
            )}
          </div>
        )}

        {/* Error badge */}
        {(upload.isError || del.isError) && (
          <div
            className="absolute top-2 right-2 flex items-center gap-1.5 px-2.5 py-1.5 rounded-lg text-xs"
            style={{ background: '#c0392b', color: 'white' }}
          >
            <AlertTriangle size={12} />
            {(upload.error as Error)?.message ?? (del.error as Error)?.message ?? '操作失败'}
          </div>
        )}
      </div>

      {/* Upload hint — hidden by default, visible on hover */}
      <p
        className="text-xs mt-2 opacity-0 group-hover:opacity-100 transition-opacity duration-200"
        style={{ color: 'var(--color-ink-muted)' }}
      >
        悬停可上传或替换 · 支持拖入 · JPG / PNG / WebP · 最大 10 MB
      </p>
      </div>{/* end outer group */}

      {/* Hidden file input */}
      <input
        ref={fileInputRef}
        type="file"
        accept="image/jpeg,image/png,image/webp,image/gif"
        className="hidden"
        onChange={handleInputChange}
      />

      {/* Confirm delete dialog */}
      {confirmOpen && (
        <div
          className="fixed inset-0 z-50 flex items-center justify-center p-4"
          style={{ background: 'rgba(26,20,16,0.5)', backdropFilter: 'blur(4px)' }}
          onClick={e => { if (e.target === e.currentTarget) setConfirmOpen(false) }}
        >
          <div
            className="w-full max-w-sm rounded-2xl p-6"
            style={{ background: 'white', boxShadow: 'var(--shadow-card-hover)' }}
          >
            <div className="flex items-start gap-4 mb-5">
              <div
                className="w-10 h-10 rounded-full flex items-center justify-center shrink-0"
                style={{ background: '#fde8d0' }}
              >
                <Trash2 size={18} style={{ color: 'var(--color-rust)' }} />
              </div>
              <div>
                <h3
                  className="font-semibold text-base mb-1"
                  style={{ fontFamily: 'var(--font-display)', color: 'var(--color-ink)' }}
                >
                  删除封面图？
                </h3>
                <p className="text-sm leading-relaxed" style={{ color: 'var(--color-ink-muted)' }}>
                  将移除《{recipeName}》的封面图，此操作不可撤销。
                </p>
              </div>
              <button
                onClick={() => setConfirmOpen(false)}
                className="ml-auto shrink-0 opacity-40 hover:opacity-70 transition-opacity"
              >
                <X size={16} />
              </button>
            </div>

            <div className="flex gap-3">
              <button
                onClick={() => setConfirmOpen(false)}
                className="flex-1 py-2.5 rounded-xl text-sm font-medium transition-opacity hover:opacity-70"
                style={{
                  background: 'var(--color-paper)',
                  color: 'var(--color-ink-muted)',
                  border: '1px solid var(--color-border)',
                }}
              >
                取消
              </button>
              <button
                onClick={confirmDelete}
                disabled={del.isPending}
                className="flex-1 py-2.5 rounded-xl text-sm font-medium transition-opacity hover:opacity-80 disabled:opacity-50 flex items-center justify-center gap-2"
                style={{ background: 'var(--color-rust)', color: 'white' }}
              >
                {del.isPending
                  ? <><Loader2 size={14} className="animate-spin" />删除中…</>
                  : '确认删除'
                }
              </button>
            </div>
          </div>
        </div>
      )}
    </>
  )
}

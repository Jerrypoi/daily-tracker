import { useEffect, useRef, useState } from 'react'

import { DEFAULT_TOPIC_COLOR } from '../api/topics'

type TopicColorPickerProps = {
  value: string
  onChange: (value: string) => void
}

const PRESET_COLORS = [
  '#3b82f6',
  '#2563eb',
  '#06b6d4',
  '#14b8a6',
  '#22c55e',
  '#f59e0b',
  '#f97316',
  '#ef4444',
  '#a855f7',
  '#ec4899',
]

function normalizeColor(value: string): string {
  return /^#[0-9a-fA-F]{6}$/.test(value) ? value.toLowerCase() : DEFAULT_TOPIC_COLOR
}

export function TopicColorPicker({ value, onChange }: TopicColorPickerProps) {
  const [hexDraft, setHexDraft] = useState(normalizeColor(value))
  const paletteInputRef = useRef<HTMLInputElement | null>(null)

  useEffect(() => {
    setHexDraft(normalizeColor(value))
  }, [value])

  function commitHex(next: string) {
    const normalized = normalizeColor(next)
    onChange(normalized)
    setHexDraft(normalized)
  }

  return (
    <div className="topic-color-picker">
      <div className="topic-color-picker-head">
        <span className="topic-color-preview" style={{ backgroundColor: normalizeColor(value) }} />
        <code className="topic-color-code">{normalizeColor(value)}</code>
        <button
          type="button"
          className="topic-color-custom"
          onClick={() => paletteInputRef.current?.click()}
        >
          <span
            className="topic-color-custom-dot"
            style={{ backgroundColor: normalizeColor(value) }}
            aria-hidden="true"
          />
          <span>Palette</span>
        </button>
        <input
          ref={paletteInputRef}
          className="topic-color-native-input"
          type="color"
          value={normalizeColor(value)}
          onChange={(event) => commitHex(event.target.value)}
          aria-label="Pick custom color"
        />
      </div>

      <div className="topic-color-swatches-wrap">
        <p className="topic-color-swatches-label">Recommended</p>
        <div className="topic-color-swatches" role="listbox" aria-label="Preset colors">
          {PRESET_COLORS.map((color) => {
            const selected = normalizeColor(value) === color
            return (
              <button
                key={color}
                type="button"
                className={`topic-color-swatch ${selected ? 'is-selected' : ''}`}
                style={{ backgroundColor: color }}
                onClick={() => commitHex(color)}
                aria-label={`Select color ${color}`}
                aria-selected={selected}
              />
            )
          })}
        </div>
      </div>

      <label>
        HEX
        <input
          value={hexDraft}
          onChange={(event) => setHexDraft(event.target.value)}
          onBlur={(event) => commitHex(event.target.value)}
          onKeyDown={(event) => {
            if (event.key === 'Enter') {
              event.preventDefault()
              commitHex(hexDraft)
            }
          }}
          placeholder="#3b82f6"
          maxLength={7}
        />
      </label>
    </div>
  )
}

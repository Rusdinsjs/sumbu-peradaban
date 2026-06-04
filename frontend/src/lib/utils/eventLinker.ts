/**
 * Helper to dynamically link occurrences of historical event titles in any string.
 * It searches the text for event titles and wraps them with standard hyperlinks to /event/[uuid].
 */
export function renderTextWithEventLinks(
  text: string | null | undefined,
  allEvents: Array<{ uuid: string; title: string }>
): string {
  if (!text) return '';
  
  // Escaping HTML to prevent XSS issues while inserting raw HTML links
  let escaped = text
    .replace(/&/g, '&amp;')
    .replace(/</g, '&lt;')
    .replace(/>/g, '&gt;')
    .replace(/"/g, '&quot;')
    .replace(/'/g, '&#039;');

  if (!allEvents || allEvents.length === 0) {
    return escaped;
  }

  // Sort events by title length descending to prevent shorter matches inside longer titles (e.g. matching "Perang" instead of "Perang Badar")
  const sortedEvents = [...allEvents].sort((a, b) => b.title.length - a.title.length);

  // Build a regex pattern from escaped titles
  const escapedTitles = sortedEvents
    .map(e => e.title.replace(/[-\/\\^$*+?.()|[\]{}]/g, '\\$&'))
    .filter(t => t.length > 0);

  if (escapedTitles.length === 0) {
    return escaped;
  }

  // Use a word-boundary-like pattern, but since event titles can have non-word characters,
  // we do a simple global match of the titles.
  const regex = new RegExp(`(${escapedTitles.join('|')})`, 'g');

  const titleToUuid = new Map(sortedEvents.map(e => [e.title, e.uuid]));

  return escaped.replace(regex, (match) => {
    const uuid = titleToUuid.get(match);
    if (uuid) {
      return `<a href="/event/${uuid}" class="text-gold-400 underline hover:text-gold-300 font-bold transition-colors">${match}</a>`;
    }
    return match;
  });
}

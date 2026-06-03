import { json } from '@sveltejs/kit';
import type { RequestHandler } from './$types';
import { writeFileSync, existsSync, mkdirSync } from 'fs';
import { join } from 'path';

export const POST: RequestHandler = async ({ request }) => {
  try {
    const formData = await request.formData();
    const file = formData.get('file') as File | null;
    
    if (!file) {
      return json({ error: 'No file uploaded' }, { status: 400 });
    }

    const buffer = Buffer.from(await file.arrayBuffer());
    
    // Generate unique filename
    const ext = file.name.split('.').pop() || 'png';
    const filename = `${Date.now()}-${Math.random().toString(36).substring(2, 9)}.${ext}`;
    
    // Save to static/uploads
    // Using process.cwd() as SvelteKit execution root
    const uploadDir = join(process.cwd(), 'static', 'uploads');
    
    if (!existsSync(uploadDir)) {
      mkdirSync(uploadDir, { recursive: true });
    }
    
    const filePath = join(uploadDir, filename);
    
    writeFileSync(filePath, buffer);

    return json({ 
      success: true, 
      url: `/uploads/${filename}`,
      title: file.name
    });
  } catch (error) {
    console.error('Upload error:', error);
    return json({ error: 'Failed to upload file' }, { status: 500 });
  }
};

import type { Meta, StoryObj } from '@storybook/vue3';

import AspectRatio from '../components/ui/aspect-ratio/AspectRatio.vue';

const meta = {
  title: 'AspectRatio',
  component: AspectRatio,
  tags: ['autodocs'],
  args: {}, // default value
} satisfies Meta<typeof AspectRatio>;

export default meta;
type Story = StoryObj<typeof AspectRatio>;

export const Primary: Story = {
    args: {
        class: 'page-container__navbar navbar container is-fluid h-28',
    }
};
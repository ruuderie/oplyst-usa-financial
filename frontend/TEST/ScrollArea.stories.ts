import type { Meta, StoryObj } from '@storybook/vue3';

import ScrollArea from '../components/ui/scroll-area/ScrollArea.vue';

const meta = {
  title: 'ScrollArea',
  component: ScrollArea,
  tags: ['autodocs'],
  args: {}, // default value
} satisfies Meta<typeof ScrollArea>;

export default meta;
type Story = StoryObj<typeof ScrollArea>;

export const Primary: Story = {
    args: {
        class: 'page-container__navbar navbar container is-fluid h-28',
    }
};
import type { Meta, StoryObj } from '@storybook/vue3';

import ScrollBar from '../components/ui/scroll-area/ScrollBar.vue';

const meta = {
  title: 'ScrollBar',
  component: ScrollBar,
  tags: ['autodocs'],
  args: {}, // default value
} satisfies Meta<typeof ScrollBar>;

export default meta;
type Story = StoryObj<typeof ScrollBar>;

export const Primary: Story = {
    args: {
        class: 'page-container__navbar navbar container is-fluid h-28',
    }
};
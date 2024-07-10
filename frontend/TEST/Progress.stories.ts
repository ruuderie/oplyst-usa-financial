import type { Meta, StoryObj } from '@storybook/vue3';

import Progress from '../components/ui/progress/Progress.vue';

const meta = {
  title: 'Progress',
  component: Progress,
  tags: ['autodocs'],
  args: {}, // default value
} satisfies Meta<typeof Progress>;

export default meta;
type Story = StoryObj<typeof Progress>;

export const Primary: Story = {
    args: {
        class: 'page-container__navbar navbar container is-fluid h-28',
    }
};
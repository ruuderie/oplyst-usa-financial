import type { Meta, StoryObj } from '@storybook/vue3';

import Skeleton from '../components/ui/skeleton/Skeleton.vue';

const meta = {
  title: 'Skeleton',
  component: Skeleton,
  tags: ['autodocs'],
  args: {}, // default value
} satisfies Meta<typeof Skeleton>;

export default meta;
type Story = StoryObj<typeof Skeleton>;

export const Primary: Story = {
    args: {
        class: 'page-container__navbar navbar container is-fluid h-28',
    }
};
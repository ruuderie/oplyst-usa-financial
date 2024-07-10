import type { Meta, StoryObj } from '@storybook/vue3';

import DialogFooter from '../components/ui/dialog/DialogFooter.vue';

const meta = {
  title: 'DialogFooter',
  component: DialogFooter,
  tags: ['autodocs'],
  args: {}, // default value
} satisfies Meta<typeof DialogFooter>;

export default meta;
type Story = StoryObj<typeof DialogFooter>;

export const Primary: Story = {
    args: {
        class: 'page-container__navbar navbar container is-fluid h-28',
    }
};